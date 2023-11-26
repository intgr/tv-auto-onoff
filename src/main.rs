mod desktop_idle;

use crate::desktop_idle::desktop_idle;

use futures::executor;
use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;
use time::macros::format_description;
use time::util::local_offset;
use time::util::local_offset::Soundness;

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

    debug!("{} {} starting...", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    executor::block_on(desktop_idle()).expect("Error from D-Bus ScreenSaver monitor");
}
