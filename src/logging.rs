use log::LevelFilter;
use simple_logger::SimpleLogger;
use time::macros::format_description;
use time::util::local_offset;
use time::util::local_offset::Soundness;

#[allow(clippy::module_name_repetitions)]
pub fn init_logging() {
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
        .expect("Error initializing logging");
}
