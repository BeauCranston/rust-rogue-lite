use macroquad::prelude::*;

pub fn conf() -> Conf {
    Conf {
        window_title: "Rogue-lite".into(),
        window_width: 1200,
        window_height: 800,
        window_resizable: false, // <- this is the key line
        ..Default::default()
    }
}
