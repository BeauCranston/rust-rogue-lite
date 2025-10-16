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

        draw_floor(&game_cam, &tile_texture);

        game_cam.end();
        game_cam.draw_to_screen();

        next_frame().await
    }
}
