use log::{Log, Metadata, Record};

pub struct MultiLogger {
    pub loggers: Vec<Box<dyn Log>>,
}

impl Log for MultiLogger {
    fn enabled(&self, record: &Metadata<'_>) -> bool {
        self.loggers.iter().any(|logger| logger.enabled(record))
    }

    fn log(&self, record: &Record<'_>) {
        self.loggers
            .iter()
            .filter(|logger| logger.enabled(record.metadata()))
            .for_each(|logger| logger.log(record));
    }

    fn flush(&self) {
        self.loggers.iter().for_each(|logger| logger.flush());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::logging::*;

    #[test]
    fn log_enabled() {
        let logger = MultiLogger {
            loggers: vec![
                Box::new(FileLogger::new(
                    Level::Debug,
                    FileLoggerOptions::new("test"),
                )),
                Box::new(StdoutLogger::new(Level::Info)),
            ],
        };
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }
}
