use crate::application::channels::ButtonEventChannel;

pub async fn button_task(button_event_channel: ButtonEventChannel) -> ! {
    loop {
        let button_event = button_event_channel.receive().await;
    }
}
