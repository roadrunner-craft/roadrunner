use log::{Level, Log, Metadata, Record};
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};

use crate::utils::time::{timestamp_format, PROGRAM_START};

const COLOR_TIMESTAMP: Color = Color::Rgb(0x96, 0x96, 0x96);
const COLOR_ERROR: Color = Color::Rgb(0xd1, 0x3d, 0x3d);
const COLOR_WARN: Color = Color::Rgb(0xf5, 0xdb, 0x47);
const COLOR_INFO: Color = Color::Rgb(0x47, 0x8d, 0xf5);
const COLOR_DEBUG: Color = Color::Rgb(0x51, 0xc9, 0x4f);
const COLOR_BODY: Color = Color::Rgb(0xff, 0xff, 0xff);

pub struct StdoutLogger {
    stdout: StandardStream,
    level: Level,
}

impl StdoutLogger {
    pub fn new(level: Level) -> Self {
        Self {
            stdout: StandardStream::stdout(ColorChoice::Always),
            level,
        }
    }

    fn set_color(&self, lock: &mut StandardStreamLock, color: Color) {
        let _ = lock.set_color(ColorSpec::new().set_fg(Some(color)));
    }
}

impl Log for StdoutLogger {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        meta.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        let mut lock = self.stdout.lock();

        // TIMESTAMP
        self.set_color(&mut lock, COLOR_TIMESTAMP);
        let _ = lock.write(format!("{} ", timestamp_format(PROGRAM_START.elapsed())).as_bytes());

        // LEVEL
        self.set_color(
            &mut lock,
            match record.level() {
                log::Level::Error => COLOR_ERROR,
                log::Level::Warn => COLOR_WARN,
                log::Level::Info => COLOR_INFO,
                log::Level::Debug => COLOR_DEBUG,
                _ => COLOR_BODY,
            },
        );
        let _ = lock.write(record.level().to_string().as_bytes());

        // MESSAGE
        self.set_color(&mut lock, COLOR_BODY);
        let _ = lock.write(format!(": {}\n", record.args()).as_bytes());

        // cleanup
        let _ = lock.reset();
    }

    fn flush(&self) {
        let mut lock = self.stdout.lock();
        lock.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_enabled_info() {
        let logger = StdoutLogger::new(log::Level::Info);
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }

    #[test]
    fn log_enabled_warn() {
        let logger = StdoutLogger::new(log::Level::Warn);
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }
}
