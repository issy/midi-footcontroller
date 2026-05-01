#![no_std]

extern crate alloc;

mod generated {
    #[path = "device/v1/device.v1.rs"]
    pub mod device_v1;
}

pub mod application;
pub mod channels;
pub mod layout;
pub mod midi;
pub mod protocol;
pub mod storage;
pub mod time;

/// A trait for types that can be converted to and from another type `T`
pub trait Convertible<T>: Sized {
    /// Convert `Self` into `T`
    fn to(self) -> T;

    /// Convert `T` into `Self`
    fn from(value: T) -> Self;
}
