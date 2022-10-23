use log::{Level, Log, Metadata, Record};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::{io, io::Write};

use crate::utils::time::{duration_since_epoch, timestamp_format, PROGRAM_START};

pub struct FileLoggerOptions {
    /// log directory
    pub directory: &'static str,
    /// current log name
    pub filename: &'static str,
    /// maximum log size in bytes
    pub max_size: u64,
    /// maximum amount of files to keep in rotation
    pub max_files: usize,
}

impl FileLoggerOptions {
    pub fn new(filename: &'static str) -> Self {
        Self {
            directory: "logs/",
            filename,
            max_size: 5 * 1024 * 1024, // 5MB
            max_files: 10,
        }
    }
}

pub struct FileLogger {
    level: Level,
    options: FileLoggerOptions,
}

impl FileLogger {
    pub fn new(level: Level, options: FileLoggerOptions) -> Self {
        Self { level, options }
    }

    /// path to the current log file
    fn log_path(&self) -> PathBuf {
        Path::new(self.options.directory).join(format!("{}.log", self.options.filename))
    }

    /// handle to the current log file, creating it if necessary
    fn log_file(&self) -> io::Result<fs::File> {
        let create_file = |_| {
            let _ = fs::create_dir_all(Path::new(self.options.directory));
            fs::File::create(self.log_path())
        };
        fs::OpenOptions::new()
            .append(true)
            .open(self.log_path())
            .or_else(create_file)
    }

    /// save current log but only keep n most recent files
    fn rotate_logs(&self) {
        // backup current log by timestamping it
        let new_file_path = Path::new(self.options.directory)
            .join(format!("{}.log", duration_since_epoch().as_millis()));
        if let Err(err) = fs::rename(self.log_path(), new_file_path) {
            panic!("could not rotate log files - {}", err);
        }

        // remove oldest log file
        let count = fs::read_dir(self.options.directory)
            .map(|dir| dir.count())
            .ok();
        if count > Some(self.options.max_files) {
            let _ = fs::read_dir(self.options.directory).map(|dir_entry: fs::ReadDir| {
                // remove first file in lexicographical order (oldest for timestamped files)
                dir_entry
                    .filter_map(|entry| entry.ok())
                    .flat_map(|entry: fs::DirEntry| {
                        entry
                            .path()
                            .file_stem()
                            .and_then(|s| s.to_str().map(|s| s.to_string()))
                    }) // iterator over file stems, as String (representing timestamps)
                    .min() // gets the first, so the oldest
                    .and_then(|stem: String| {
                        fs::remove_file(
                            Path::new(self.options.directory).join(format!("{}.log", stem)),
                        )
                        .ok()
                    })
            });
        }
    }
}

/// rotating file implementation for log
impl Log for FileLogger {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        meta.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        if let Ok(file) = self.log_file().as_mut() {
            let data = format!(
                "{} {}: {}\n",
                timestamp_format(PROGRAM_START.elapsed()),
                record.level(),
                record.args()
            );

            let _ = file.write(data.as_bytes());

            if let Ok(size) = file.metadata().map(|meta| meta.len()) {
                if size >= self.options.max_size {
                    self.rotate_logs();
                }
            }
        } else {
            println!("ERROR: could not open log file");
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_enabled_debug() {
        let logger = FileLogger::new(log::Level::Debug, FileLoggerOptions::new("test"));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }

    #[test]
    fn log_enabled_error() {
        let logger = FileLogger::new(log::Level::Error, FileLoggerOptions::new("test"));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }
}
