use macroquad::prelude::*;

// A custom camera that will render in a specific aspect ratio, as well as use units instead of
// pixels
pub struct PixelCamera {
    //if for example ppu = 8, there will be 8 pixels per unit. Meaning 20 units for 160 pixels
    pub ppu: f32,
    //native resolution for game
    pub virtual_w: f32,
    pub virtual_h: f32,
    //a texture to render to that will later be upscaled to match the screens resolution
    pub render_target: RenderTarget,
    //the camera object
    pub camera2d: Camera2D,
}

impl PixelCamera {
    pub fn new(ppu: f32, virtual_w: f32, virtual_h: f32) -> Self {
        //create a render target the size that the games resolution should be
        let rt = render_target(virtual_w as u32, virtual_h as u32);
        //filtermode nearest will ensure clear pixels when upscaled
        rt.texture.set_filter(FilterMode::Nearest);

        //clone the render target for the camera2D object
        let rt_clone = rt.clone();

        PixelCamera {
            ppu: ppu,
            virtual_w: virtual_w,
            virtual_h: virtual_h,
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
            camera2d: Camera2D {
                target: vec2(virtual_w * 0.5, virtual_h * 0.5), // center the camera in your world units
                zoom: vec2(2.0 / virtual_w, 2.0 / virtual_h),
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
        let sw = screen_width();
        let sh = screen_height();

        // Largest integer scale that fits both width and height
        let scale = (sw / self.virtual_w)
            .floor()
            .min((sh / self.virtual_h).floor())
            .max(1.0);

        let dest_w = self.virtual_w * scale;
        let dest_h = self.virtual_h * scale;

        // Centering (letterboxing)
        let offset_x = (sw - dest_w) * 0.5;
        let offset_y = (sh - dest_h) * 0.5;

        // Optional: draw bars (they’ll be whatever you clear the backbuffer with)
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
    } // integer-scale + letterbox
}
