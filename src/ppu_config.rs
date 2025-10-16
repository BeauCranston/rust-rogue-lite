use macroquad::{
    color::WHITE,
    math::{Vec2, vec2},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};

pub static PPU: f32 = 8.0;

pub fn units_to_vpx(u: f32) -> f32 {
    u * PPU
}

pub fn vpx_to_units(px: f32) -> f32 {
    px / PPU
}

pub fn draw_tex_units(tex: &Texture2D, pos_units: Vec2, params: Option<DrawTextureParams>) {
    draw_texture_ex(
        tex,
        pos_units.x,
        pos_units.y,
        WHITE,
        params.unwrap_or_default(),
    );
}
