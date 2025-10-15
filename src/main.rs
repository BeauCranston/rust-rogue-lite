use game_camera::*;
use macroquad::{
    audio::{self, play_sound, stop_sound},
    miniquad::window::dpi_scale,
    prelude::*,
    rand::rand,
};
use window_conf::conf;

mod game_camera;
mod window_conf;

#[macroquad::main(conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let tile_texture = load_texture("tile.png").await.unwrap();
    tile_texture.set_filter(FilterMode::Nearest);

    let mut game_cam = PixelCamera::new(8.0, 240.0, 160.0);

    loop {
        game_cam.begin();
        // Your world is now in "units" — not pixels.
        // Draw a single 8x8 *unit* tile at position (10,10) units:
        draw_texture_ex(
            &tile_texture,
            10.0, // x in units
            10.0, // y in units
            WHITE,
            DrawTextureParams {
                // IMPORTANT: size in *units*, not pixels.
                // If the source image is 8x8 px, this will scale it to 8x8 units.
                dest_size: Some(vec2(8.0, 8.0)),
                ..Default::default()
            },
        );

        // Another tile, half the size (4x4 units) at a different location:
        draw_texture_ex(
            &tile_texture,
            25.0,
            12.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(4.0, 4.0)),
                ..Default::default()
            },
        );

        game_cam.end();
        game_cam.draw_to_screen();

        next_frame().await
    }
}
