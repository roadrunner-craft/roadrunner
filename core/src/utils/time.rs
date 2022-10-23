use std::time::{Duration, Instant, SystemTime};

lazy_static! {
    pub static ref PROGRAM_START: Instant = Instant::now();
}

/// current amount of milliseconds elapsed since epoch
pub fn duration_since_epoch() -> Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}

pub fn timestamp_format(elapsed: Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        elapsed.as_secs() / 3600,
        elapsed.as_secs() / 60 % 60,
        elapsed.as_secs() % 60,
        elapsed.subsec_millis()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_since_format() {
        assert_eq!("00:00:00.000", timestamp_format(Duration::new(0, 0)));
        assert_eq!("00:00:00.999", timestamp_format(Duration::from_millis(999)));
        assert_eq!("00:00:01.000", timestamp_format(Duration::from_secs(1)));
        assert_eq!("00:00:59.000", timestamp_format(Duration::from_secs(59)));
        assert_eq!("00:01:00.000", timestamp_format(Duration::from_secs(60)));
        assert_eq!(
            "00:59:00.000",
            timestamp_format(Duration::from_secs(59 * 60))
        );
        assert_eq!(
            "01:00:00.000",
            timestamp_format(Duration::from_secs(60 * 60))
        );
        assert_eq!(
            "99:00:00.000",
            timestamp_format(Duration::from_secs(99 * 60 * 60))
        );
    }
}
