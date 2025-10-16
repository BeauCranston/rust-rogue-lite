use game_camera::*;
use macroquad::{
    audio::{self, play_sound, stop_sound},
    miniquad::window::dpi_scale,
    prelude::*,
    rand::rand,
};
use window_conf::conf;

use crate::floor::draw_floor;

mod floor;
mod game_camera;
mod ppu_config;
mod window_conf;
#[macroquad::main(conf)]

async fn main() {
    set_pc_assets_folder("assets");
    let tile_texture = load_texture("tile.png").await.unwrap();
    tile_texture.set_filter(FilterMode::Nearest);

    //create a game camera that works with units instead of pixels
    // 8 PPU, 240x160 native resolution
    let mut game_cam = GameCamera::new(240.0, 160.0);

    loop {
        game_cam.begin();

        // Another tile, half the size (4x4 units) at a different location:
        // draw_texture_ex(
        //     &tile_texture,
        //     25.0,
        //     12.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(vec2(1.0, 1.0)),
        //         ..Default::default()
        //     },
        // );

        draw_floor(&game_cam, &tile_texture);

        game_cam.end();
        game_cam.draw_to_screen();

        next_frame().await
    }
}
