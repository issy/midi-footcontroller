#![no_std]

extern crate alloc;

pub mod generated {
    #[path = "device/v1/device.v1.rs"]
    pub mod device_v1;
}

pub mod layout;
pub mod midi;
pub mod protocol;
