use crate::storage::state::Presets;
use core::fmt::Debug;

pub mod state;

#[derive(Debug)]
pub enum StorageManagerLoadError {
    ErrorReadingFromStorage,
    NoValueStored,
    ErrorDeserializingData,
}

#[derive(Debug)]
pub enum StorageManagerSaveError {
    ErrorDeserializingData,
    ErrorWritingToStorage,
}

pub trait StorageManager {
    fn load_presets(&self) -> Result<Presets, StorageManagerLoadError>;
    fn save_presets(&mut self, presets: &Presets) -> Result<(), StorageManagerSaveError>;
}
