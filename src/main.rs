use std::env;
use std::net::IpAddr;
use std::str::FromStr;

use futures::executor;
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;
use time::macros::format_description;
use time::util::local_offset;
use time::util::local_offset::Soundness;

use crate::desktop_idle::desktop_idle;
use crate::tv_manager::TvManager;

mod bravia;
mod desktop_idle;
mod tv_manager;

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

    // TODO Loosen coupling between desktop_idle and BraviaClient
    executor::block_on(desktop_idle(tv)).expect("Error from D-Bus ScreenSaver monitor");
}
