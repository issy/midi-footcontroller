use crate::application::channels::DisplayStateUpdateChannel;
use crate::application::state::Application;
use crate::layout::DisplayLayout;
use crate::midi::{MidiReader, MidiWriter};
use crate::storage::StorageManager;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;

/// Read app state updates and render them to the display
pub async fn display_task<
    'a,
    D: DrawTarget<Color = Rgb565>,
    MR: MidiReader,
    MW: MidiWriter,
    SM: StorageManager,
>(
    channel: DisplayStateUpdateChannel,
    application: Application<'a, D, MR, MW, SM>,
) -> ! {
    let mut display_1_layout = DisplayLayout::new(application.displays.display_1);
    let mut display_2_layout = DisplayLayout::new(application.displays.display_2);
    let mut display_3_layout = DisplayLayout::new(application.displays.display_3);
    let mut display_4_layout = DisplayLayout::new(application.displays.display_4);

    loop {
        let update_message = channel.receive().await;
        let target = match update_message.display_index {
            0 => &mut display_1_layout,
            1 => &mut display_2_layout,
            2 => &mut display_3_layout,
            3 => &mut display_4_layout,
            _ => continue, // Invalid display index, ignore the message
        };
    }
}
