use foundation::time::TimeSource;
use web_sys::js_sys::Date;

#[derive(Default)]
pub struct BrowserTimeSource;

impl TimeSource for BrowserTimeSource {
    fn now(&self) -> u64 {
        Date::now() as u64
    }
}
