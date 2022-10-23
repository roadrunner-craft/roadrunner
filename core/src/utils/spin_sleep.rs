use std::thread;
use std::time::{Duration, Instant};

const SLEEP_ACCURACY: u32 = 6_000_000;

pub fn sleep(duration: Duration) {
    let start = Instant::now();
    let spin_duration = Duration::new(0, SLEEP_ACCURACY);

    if duration > spin_duration {
        thread::sleep(duration - spin_duration);
    }

    while start.elapsed() < duration {
        thread::yield_now();
    }
}
