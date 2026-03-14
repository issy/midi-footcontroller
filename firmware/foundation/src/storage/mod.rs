use crate::storage::state::Presets;

pub mod state;

pub trait StorageManager {
    fn load_presets(&self) -> Presets;
    fn save_presets(&mut self, presets: &Presets);
}
