use foundation::application::time::TimeSource;

#[derive(Default)]
pub struct EmbassyTimeSource;

impl TimeSource for EmbassyTimeSource {
    fn now(&self) -> u64 {
        embassy_time::Instant::now().as_millis()
    }
}
