use embedded_graphics::draw_target::DrawTarget;

struct Displays<D: DrawTarget> {
    display_1: D,
    display_2: D,
    display_3: D,
    display_4: D,
}

pub struct AppState<D: DrawTarget> {
    displays: Displays<D>,
    // TODO: Add MIDI streams
    // TODO: Add protocol streams
    // TODO: Add buttons
}

impl<D: DrawTarget> AppState<D> {
    pub fn new(display_1: D, display_2: D, display_3: D, display_4: D) -> Self {
        Self {
            displays: Displays {
                display_1,
                display_2,
                display_3,
                display_4,
            },
        }
    }
}
