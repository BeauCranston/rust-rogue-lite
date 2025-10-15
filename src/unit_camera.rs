use macroquad::prelude::*;

pub fn make_unit_camera() -> Camera2D {
    let sw = screen_width();
    let sh = screen_height();

    let vw = sw / PPU; // virtual width in units
    let vh = sh / PPU; // virtual height in units

    // Camera2D::zoom maps world units to NDC (-1..1). Use 2.0/virtual_size
    Camera2D {
        target: vec2(vw * 0.5, vh * 0.5), // center the camera in your world units
        zoom: vec2(2.0 / vw, 2.0 / vh),
        ..Default::default()
    }
}
