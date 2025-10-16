use macroquad::prelude::*;

use crate::{
    game_camera::GameCamera,
    ppu_config::{draw_tex_units, units_to_vpx},
};

pub fn draw_floor(game_cam: &GameCamera, tile_texture: &Texture2D) {
    let screen_size = game_cam.screen_size_units();
    let screen_area = screen_size.x * screen_size.y;
    let mut py = 0.0;
    let mut px = 0.0;

    for unit in 0..screen_area as i32 {
        // if unit is 10 and screen_size is 5, py would be 2
        py = (unit as f32 / screen_size.x as f32).floor();
        // if unit is 13 and screen size is 5 x would be 13 - (5*2) = 3 which would put the x at 3
        px = unit as f32 - (screen_size.x * py);
        draw_tex_units(
            tile_texture,
            vec2(px, py),
            Some(DrawTextureParams {
                dest_size: Some(vec2(1.0, 1.0)),
                ..Default::default()
            }),
        );
    }

    // for x in 0..game_cam.() as i32 {
    //     for y in 0..game_cam.screen_height_units() as i32 {
    //         draw_texture_ex(
    //             tile_texture,
    //             units_to_vpx(x as f32),
    //             units_to_vpx(y as f32),
    //             WHITE,
    //             DrawTextureParams {
    //                 dest_size: Some(vec2(1.0, 1.0)),
    //                 ..Default::default()
    //             },
    //         );
    //     }
    // }
}
