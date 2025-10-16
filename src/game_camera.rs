use macroquad::prelude::*;

use crate::ppu_config::vpx_to_units;

// A custom camera that will render in a specific aspect ratio, as well as use units instead of
// pixels
pub struct GameCamera {
    //if for example ppu = 8, there will be 8 pixels per unit. Meaning 20 units for 160 pixels
    //native resolution for game
    virtual_w: f32,
    virtual_h: f32,
    units_w: f32,
    units_h: f32,
    //a texture to render to that will later be upscaled to match the screens resolution
    render_target: RenderTarget,
    //the camera object
    camera2d: Camera2D,
}

impl GameCamera {
    pub fn new(virtual_w: f32, virtual_h: f32) -> Self {
        //create a render target the size that the games resolution should be
        let rt = render_target(virtual_w as u32, virtual_h as u32);
        //filtermode nearest will ensure clear pixels when upscaled
        rt.texture.set_filter(FilterMode::Nearest);

        //clone the render target for the camera2D object
        let rt_clone = rt.clone();
        let units_w = vpx_to_units(virtual_w);
        let units_h = vpx_to_units(virtual_h);

        Self {
            virtual_w: virtual_w,
            virtual_h: virtual_h,
            units_w: units_w,
            units_h: units_h,
            render_target: rt,
            // World coordinates are mapped to NDC(normalized device coordinates)
            // The formula for this mapping is ((x || y) - target ) * zoom
            // NDC coordinates are in the range of -1.0 - +1.0. Which is essentially a 2x2 grid.
            // Zoom is calculated with 2.0/(virtual_w||virtual_h) to fit the virtual units within
            // the 2x2 grid
            // if a world has 240 units in length, an object has an x of 180 in world units, the camera's target is in the center at 120 then:
            //  n = (180 - target.x) * zoom.x
            //  n = (180 - 120) * (2.0/240)
            //  n = 60 * 0.0083333333
            //  n = 0.5.
            //  This means that the objects n position is directly in the center between the righ edge of the screen, and the center
            //  The camera is where we can define how much 1 unit is and how many pixels it takes
            //  up.
            camera2d: Camera2D {
                target: vec2(units_w * 0.5, units_h * 0.5), // center the camera in your world units
                zoom: vec2(2.0 / units_w, 2.0 / units_h),
                render_target: Some(rt_clone),
                ..Default::default()
            },
        }
    }
    pub fn begin(&mut self) {
        set_camera(&self.camera2d);
    }
    pub fn end(&self) {
        set_default_camera();
    }

    pub fn draw_to_screen(&self) {
        //get the width and height of the screen
        let sw = screen_width();
        let sh = screen_height();

        // Largest integer scale that fits both width and height
        // if the width scales up 8 times, and the height scales up 6,
        // we will scale up both 6 times to maintain the aspect ratio and not overflow.
        // We then take the max of scale or 1.0 to ensure we aren't scaling down.
        let scale = (sw / self.virtual_w)
            .floor()
            .min((sh / self.virtual_h).floor())
            .max(1.0);

        //final size of the scaled up canvas
        let dest_w = self.virtual_w * scale;
        let dest_h = self.virtual_h * scale;

        //centering the final canvas that will be wrapped with bars to fill the difference of the
        //screen
        let offset_x = (sw - dest_w) * 0.5;
        let offset_y = (sh - dest_h) * 0.5;

        clear_background(BLACK);

        // Draw the off-screen texture to the backbuffer, scaled & centered
        draw_texture_ex(
            &self.render_target.texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                // If your result appears vertically flipped on your platform/driver,
                // set flip_y = true (there have been platform-specific flips with targets). :contentReference[oaicite:3]{index=3}
                // flip_y: true,
                ..Default::default()
            },
        );
    }
    pub fn screen_size(&self) -> Vec2 {
        Vec2 {
            x: self.virtual_w,
            y: self.virtual_h,
        }
    }
    pub fn screen_size_units(&self) -> Vec2 {
        Vec2 {
            x: self.units_w,
            y: self.units_h,
        }
    }
}
