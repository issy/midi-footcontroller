use core::time::Duration;

pub trait TimeSource {
    fn now(&self) -> u64;

    fn duration(&self, from: u64, to: u64) -> Duration {
        Duration::from_millis(to - from)
    }
}
