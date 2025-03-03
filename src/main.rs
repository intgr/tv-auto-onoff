#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::env;
use std::iter::Iterator;
use std::net::IpAddr;
use std::pin::pin;
use std::str::FromStr;
use std::time::Duration;

use async_io::Timer;
use futures::executor;
use futures::StreamExt;
use futures_concurrency::stream::Merge;
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
    let idle_events = pin!(desktop_events()
        .await
        .expect("Error monitoring desktop events on D-Bus"));

    let keepalive_events =
        Timer::interval(Duration::from_secs(KEEPALIVE_INTERVAL)).map(|_| LoopEvent::Keepalive);

    let mut merged_events = (idle_events, keepalive_events).merge();

    let mut current_blanked: Option<bool> = None;

    while let Some(item) = merged_events.next().await {
        match item {
            LoopEvent::ScreenSaver(blanked) => {
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
            LoopEvent::Keepalive => {
                if current_blanked == Some(false) {
                    trace!("Keep-alive");
                    tv.keepalive();
                } else {
                    debug!("Skipping keep-alive while blanked");
                }
            }
            LoopEvent::Noop => {}
        }
    }

    panic!("Events stream ended unexpectedly");
}
