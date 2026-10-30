use mod_api::bindings::{
    Guest,
    cinnabar::extension::{hud, input},
};

struct Hello;

impl Guest for Hello {
    fn init() {
        let _ = hud::set_label("Hello from a Cinnabar mod. Press the demo key.");
    }

    fn frame() {
        if input::demo_pressed() {
            let _ = hud::set_label("Hello! The mod received your keybind.");
        }
    }
}

mod_api::bindings::export!(Hello with_types_in mod_api::bindings);
