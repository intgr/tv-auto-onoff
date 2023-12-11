#![warn(clippy::pedantic)]

use std::env;
use std::iter::Iterator;
use std::net::IpAddr;
use std::pin::pin;
use std::str::FromStr;

use futures::StreamExt;
use futures::{executor, select};
use futures_time::stream::interval;
use futures_time::time::Duration;
use log::{debug, trace};

use crate::desktop_idle::desktop_events;
use crate::logging::init_logging;
use crate::tv_manager::TvManager;
use crate::util::BoxError;

mod bravia;
mod desktop_idle;
mod logging;
mod tv_manager;
mod util;

fn main() {
    init_logging();

    debug!("Starting {} {}...", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    // Set up BraviaClient
    // TODO better CLI handling
    let args: Vec<String> = env::args().collect();
    let ip_str = args.get(1).expect("Provide Bravia IP address as argument");
    let ip = IpAddr::from_str(ip_str).expect("Invalid IP address");
    let tv = TvManager::new(ip);

    executor::block_on(main_loop(tv)).expect("Error running main loop");
}

pub enum LoopEvent {
    ScreenSaver(bool),
    Keepalive,
    Noop,
}

/// Ping TV every 10 minutes.
const KEEPALIVE_INTERVAL: u64 = 600;

async fn main_loop(tv: TvManager) -> Result<(), BoxError> {
    let mut keepalive_ping = interval(Duration::from_secs(KEEPALIVE_INTERVAL)).fuse();
    let mut idle_monitor = pin!(desktop_events()
        .await
        .expect("Error monitoring desktop events on D-Bus")
        .fuse());
    let mut current_blanked: Option<bool> = None;

    loop {
        let item = select! {
            item = idle_monitor.next() => item,
            _item = keepalive_ping.next() => Some(LoopEvent::Keepalive),
        };

        match item {
            Some(LoopEvent::ScreenSaver(blanked)) => {
                if current_blanked == Some(blanked) {
                    continue; // Nothing changed
                }
                current_blanked = Some(blanked);

                debug!("Screen blanked: {:?}", blanked);
                if blanked {
                    tv.turn_off();
                } else {
                    tv.turn_on();
                }
            }
            Some(LoopEvent::Keepalive) => {
                if current_blanked == Some(false) {
                    trace!("Keep-alive");
                    tv.keepalive();
                } else {
                    debug!("Skipping keep-alive while blanked");
                }
            }
            Some(LoopEvent::Noop) => {}
            None => break,
        }
    }

    // TODO should be error?
    Ok(())
}
