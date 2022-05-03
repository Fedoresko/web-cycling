use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub struct Framebuffer {
    pub framebuffer: Option<WebGlFramebuffer>,
    pub color_texture: Option<WebGlTexture>,
    pub depth_texture: Option<WebGlTexture>,
}

impl Framebuffer {
    pub fn create_texture_frame_buffer(
        w: i32,
        h: i32,
        gl: &WebGlRenderingContext,
    ) -> (Option<WebGlTexture>, Option<WebGlFramebuffer>) {
        let target_texture = gl.create_texture();
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, target_texture.as_ref());

        // define size and format of level 0
        let level = 0;
        let internal_format = GL::RGBA;
        let border = 0;
        let format = GL::RGBA;
        let gltype = GL::UNSIGNED_BYTE;
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D,
            level,
            internal_format as i32,
            w,
            h,
            border,
            format,
            gltype,
            None,
        )
            .expect("Cannot set texture for FBO");

        // set the filtering so we don't need mips
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

        let depth_buffer = gl.create_renderbuffer().unwrap();
        gl.bind_renderbuffer(GL::RENDERBUFFER, Some(&depth_buffer));
        gl.renderbuffer_storage(GL::RENDERBUFFER, GL::DEPTH_COMPONENT16, w, h);
        // Create and bind the framebuffer
        let fb = gl.create_framebuffer();
        gl.bind_framebuffer(GL::FRAMEBUFFER, fb.as_ref());

        // attach the texture as the first color attachment
        let attachment_point = GL::COLOR_ATTACHMENT0;
        gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            attachment_point,
            GL::TEXTURE_2D,
            target_texture.as_ref(),
            level,
        );
        gl.framebuffer_renderbuffer(
            GL::FRAMEBUFFER,
            GL::DEPTH_ATTACHMENT,
            GL::RENDERBUFFER,
            Some(&depth_buffer),
        );

        (Some(target_texture.unwrap()), Some(fb.unwrap()))
    }
}
