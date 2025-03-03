use std::io::stdout;
use std::os::fd::AsRawFd;
use std::{env, mem};

use log::LevelFilter;
use simple_logger::SimpleLogger;
use time::macros::format_description;

/// Is current process directly connected to the systemd journal?
/// Somewhat inspired by `systemd-journal-logger`.
#[cfg(target_os = "linux")]
pub fn connected_to_journal() -> bool {
    let Some(journal_stream) = env::var_os("JOURNAL_STREAM") else {
        return false;
    };
    let mut stat: libc::stat = unsafe { mem::zeroed() };
    let result = unsafe { libc::fstat(stdout().as_raw_fd(), &mut stat) };
    if result != 0 {
        return false;
    }
    return journal_stream.to_string_lossy() == format!("{}:{}", stat.st_dev, stat.st_ino);
}

#[cfg(not(target_os = "linux"))]
pub fn connected_to_journal() -> bool {
    false
}

pub fn init_logging() {
    // Omit log timestamps and colors when running under systemd.
    let builder = if connected_to_journal() {
        SimpleLogger::new().without_timestamps()
    } else {
        SimpleLogger::new()
            .with_local_timestamps()
            .with_timestamp_format(format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ))
            .with_colors(true)
    };

    builder
        .with_level(LevelFilter::Debug)
        .init()
        .expect("Error initializing logging");
}
