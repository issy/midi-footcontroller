use crate::application::channels::{StorageStateEvent, StorageStateUpdateChannel};
use crate::storage::StorageManager;

async fn storage_read_task<SM: StorageManager>(
    storage_manager: SM,
    storage_state_event_channel: StorageStateUpdateChannel,
) -> ! {
    let _presets = storage_manager
        .load_presets()
        .expect("Failed to load presets from storage");
    loop {
        let event = storage_state_event_channel.receive().await;
        match event {
            StorageStateEvent::PresetUpdate { .. } => todo!(),
            StorageStateEvent::SavePreset => todo!(),
        }
    }
}
