use crate::application::channels::ButtonEventChannel;
use crate::application::state::Application;
use crate::midi::{MidiReader, MidiWriter};
use crate::storage::StorageManager;
use embedded_graphics::draw_target::DrawTarget;

pub async fn button_task<'a, D: DrawTarget, MR: MidiReader, MW: MidiWriter, SM: StorageManager>(
    button_event_channel: ButtonEventChannel,
) -> ! {
    loop {
        let button_event = button_event_channel.receive().await;
    }
}
