use foundation::storage::state::Presets;
use foundation::storage::{StorageManager, StorageManagerLoadError, StorageManagerSaveError};

#[derive(Default)]
pub struct FakeStorageManager;

impl StorageManager for FakeStorageManager {
    fn load_presets(&self) -> Result<Presets, StorageManagerLoadError> {
        Ok(heapless::Vec::new())
    }

    fn save_presets(&mut self, presets: &Presets) -> Result<(), StorageManagerSaveError> {
        // Do nothing
        Ok(())
    }
}
