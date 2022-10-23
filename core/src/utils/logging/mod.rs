mod file_logger;
mod multi_logger;
mod stdout_logger;

pub use file_logger::{FileLogger, FileLoggerOptions};
pub use log::{Level, Log};
pub use stdout_logger::StdoutLogger;

use log::LevelFilter;
use multi_logger::MultiLogger;

pub fn init(loggers: Vec<Box<dyn Log>>) {
    let _ = log::set_boxed_logger(Box::new(MultiLogger { loggers }));
    log::set_max_level(LevelFilter::max());
}
