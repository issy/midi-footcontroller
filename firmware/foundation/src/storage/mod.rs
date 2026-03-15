use crate::storage::state::Presets;
use core::fmt::Debug;

pub mod state;

pub trait StorageManager {
    type Error: Debug;

    fn load_presets(&self) -> Result<Presets, Self::Error>;
    fn save_presets(&mut self, presets: &Presets) -> Result<(), Self::Error>;
}
