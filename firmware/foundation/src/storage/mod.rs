use crate::storage::state::MAX_PRESETS;
use crate::storage::state::StoredPreset;
use heapless::Vec;

pub mod state;

pub trait StorageManager {
    fn load_presets(&self) -> Vec<StoredPreset, MAX_PRESETS>;
    fn save_presets(&mut self, presets: &Vec<StoredPreset, MAX_PRESETS>);
}
