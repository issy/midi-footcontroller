use core::marker::PhantomData;
use foundation::storage::state::Presets;
use foundation::storage::{StorageManager, StorageManagerLoadError, StorageManagerSaveError};

#[derive(Default)]
pub struct FakeStorageManager<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> StorageManager for FakeStorageManager<'a> {
    fn load_presets(&self) -> Result<Presets, StorageManagerLoadError> {
        Ok(heapless::Vec::new())
    }

    fn save_presets(&mut self, presets: &Presets) -> Result<(), StorageManagerSaveError> {
        // Do nothing
        Ok(())
    }
}
