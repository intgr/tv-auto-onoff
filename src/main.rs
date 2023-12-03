use std::env;
use std::iter::Iterator;
use std::net::IpAddr;
use std::str::FromStr;

use futures::executor;
use futures::StreamExt;
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;
use time::macros::format_description;
use time::util::local_offset;
use time::util::local_offset::Soundness;

use crate::desktop_idle::{desktop_events, DesktopEvent};
use crate::tv_manager::TvManager;
use crate::util::BoxError;

mod bravia;
mod desktop_idle;
mod tv_manager;
mod util;

fn main() {
    // time-rs is silly...
    unsafe { local_offset::set_soundness(Soundness::Unsound) };

    SimpleLogger::new()
        .with_local_timestamps()
        .with_timestamp_format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ))
        .with_level(LevelFilter::Debug)
        .with_colors(true)
        .init()
        .unwrap();

    debug!("Starting {} {}...", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    // Set up BraviaClient
    // TODO better CLI handling
    let args: Vec<String> = env::args().collect();
    let ip_str = args.get(1).expect("Provide Bravia IP address as argument");
    let ip = IpAddr::from_str(ip_str).expect("Invalid IP address");
    let tv = TvManager::new(ip);

    executor::block_on(main_loop(tv)).expect("Error running main loop");
}

async fn main_loop(tv: TvManager) -> Result<(), BoxError> {
    let mut idle_monitor = desktop_events()
        .await
        .expect("Error monitoring desktop events on D-Bus");

    while let Some(event) = idle_monitor.next().await {
        let DesktopEvent::ScreenSaver(blanked) = event;
        debug!("ScreenSaver active: {:?}", blanked);

        if blanked {
            tv.turn_off();
        } else {
            tv.turn_on();
        }
    }

    // TODO should be error?
    Ok(())
}
