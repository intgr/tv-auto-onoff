use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;
use time::macros::format_description;

fn main() {
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
}
