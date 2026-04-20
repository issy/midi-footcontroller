use crate::layout::DisplayText;
use crate::midi::MidiPacket;
use crate::protocol::Colour;

#[cfg(target_arch = "wasm32")]
pub struct Inner<T, const N: usize> {
    tx: async_channel::Sender<T>,
    rx: async_channel::Receiver<T>,
}

#[cfg(target_arch = "wasm32")]
impl<T, const N: usize> Inner<T, N> {
    fn new() -> Self {
        let (tx, rx) = async_channel::bounded(N);
        Inner { tx, rx }
    }
}

#[cfg(not(target_arch = "wasm32"))]
type Inner<T, const N: usize> = embassy_sync::channel::Channel<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    T,
    N,
>;

pub struct AppChannel<T, const N: usize> {
    inner: Inner<T, N>,
}

impl<T, const N: usize> AppChannel<T, N> {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }

    pub async fn send(&self, value: T) {
        #[cfg(target_arch = "wasm32")]
        self.inner.tx.send(value).await.unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        self.inner.send(value).await;
    }

    pub async fn receive(&self) -> T {
        #[cfg(target_arch = "wasm32")]
        // TODO: Don't unwrap here
        return self.inner.rx.recv().await.unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        return self.inner.receive().await;
    }
}

pub type MidiOutChannel = AppChannel<MidiPacket, 128>;

pub enum DisplayIdentifier {
    Display1,
    Display2,
    Display3,
    Display4,
}

pub struct DisplayStateUpdateMessage {
    pub(crate) display_identifier: DisplayIdentifier,
    pub(crate) top_row_text: DisplayText,
    pub(crate) top_row_color: Colour,
    pub(crate) bottom_row_text: DisplayText,
    pub(crate) bottom_row_color: Colour,
}

pub type DisplayStateUpdateChannel = AppChannel<DisplayStateUpdateMessage, 16>;

#[derive(Debug, Copy, Clone)]
pub enum ButtonIdentifier {
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}

#[derive(Debug, Copy, Clone)]
pub enum ButtonEvent {
    Pressed { button_identifier: ButtonIdentifier },
    Released { button_identifier: ButtonIdentifier },
}

pub type ButtonEventChannel = AppChannel<ButtonEvent, 16>;

// TODO: Add channel for state updates
pub enum StorageStateEvent {
    PresetUpdate {
        preset_name: DisplayText,
        // Display 1
        display_1_top_row_text: Option<DisplayText>,
        display_1_top_row_color: Option<Colour>,
        display_1_bottom_row_text: Option<DisplayText>,
        display_1_bottom_row_color: Option<Colour>,
        // Display 2
        display_2_top_row_text: Option<DisplayText>,
        display_2_top_row_color: Option<Colour>,
        display_2_bottom_row_text: Option<DisplayText>,
        display_2_bottom_row_color: Option<Colour>,
        // Display 3
        display_3_top_row_text: Option<DisplayText>,
        display_3_top_row_color: Option<Colour>,
        display_3_bottom_row_text: Option<DisplayText>,
        display_3_bottom_row_color: Option<Colour>,
        // Display 4
        display_4_top_row_text: Option<DisplayText>,
        display_4_top_row_color: Option<Colour>,
        display_4_bottom_row_text: Option<DisplayText>,
        display_4_bottom_row_color: Option<Colour>,
        // TODO: Button actions
    },
    SavePreset,
}

pub type StorageStateUpdateChannel = AppChannel<StorageStateEvent, 16>;
