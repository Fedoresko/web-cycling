use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

pub struct Framebuffer {
    pub framebuffer: Option<WebGlFramebuffer>,
    pub color_texture: Option<WebGlTexture>,
    pub depth_texture: Option<WebGlTexture>,
}

const RENDERBUFFER: i32 = 0;
const COLORBUFFER: i32 = 1;

impl Framebuffer {
    pub fn create_framebuffers_multisampling(w: i32,
                                             h: i32,
                                             gl: &WebGl2RenderingContext,) -> (Option<WebGlTexture>, Option<WebGlFramebuffer>, Option<WebGlFramebuffer>) {
        let renderbuffer = Self::create_msaa_fbo(w, h, gl);

        // let depth_texture = gl.create_texture();
        // gl.bind_texture(GL::TEXTURE_2D, depth_texture.as_ref());
        // gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, gl.NEAREST as i32);
        // gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, gl.NEAREST as i32);
        // gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        //     GL::TEXTURE_2D, 0, GL::DEPTH_COMPONENT16, w, h, 0, GL::DEPTH_COMPONENT, GL::UNSIGNED_SHORT, None).expect("Cannot set texture for FBO");
        // gl.bind_texture(GL::TEXTURE_2D, None);
        //
        // let depth_renderbuffer = gl.create_renderbuffer();
        // gl.bind_renderbuffer(GL::RENDERBUFFER, depth_renderbuffer);
        // gl.renderbuffer_storage_multisample(GL::RENDERBUFFER, 4, GL::DEPTH_COMPONENT16, w, h);


        let colorbuffer = gl.create_framebuffer();
        let texture = gl.create_texture();
        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            GL::TEXTURE_2D, 0, GL::RGBA as i32, w, h, 0, GL::RGBA, GL::UNSIGNED_BYTE, None).expect("Cannot set texture for FBO");
        gl.bind_texture(GL::TEXTURE_2D, None);

        gl.bind_framebuffer(GL::FRAMEBUFFER, colorbuffer.as_ref());
        gl.framebuffer_texture_2d(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::TEXTURE_2D, texture.as_ref(), 0);
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        (Some(texture.unwrap()), Some(renderbuffer.unwrap()), Some(colorbuffer.unwrap()))
    }

    pub fn create_msaa_fbo(w: i32, h: i32, gl: &WebGl2RenderingContext) -> Option<WebGlFramebuffer> {
        let renderbuffer = gl.create_framebuffer();

        let color_renderbuffer = gl.create_renderbuffer();
        gl.bind_renderbuffer(GL::RENDERBUFFER, color_renderbuffer.as_ref());
        gl.renderbuffer_storage_multisample(GL::RENDERBUFFER, 4, GL::RGBA8, w, h);

        let depth_buffer = gl.create_renderbuffer();
        gl.bind_renderbuffer(GL::RENDERBUFFER, depth_buffer.as_ref());
        gl.renderbuffer_storage_multisample(GL::RENDERBUFFER, 4, GL::DEPTH_COMPONENT24, w, h);

        gl.bind_framebuffer(GL::FRAMEBUFFER, renderbuffer.as_ref());
        gl.framebuffer_renderbuffer(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::RENDERBUFFER, color_renderbuffer.as_ref());
        gl.framebuffer_renderbuffer(GL::FRAMEBUFFER, GL::DEPTH_ATTACHMENT, GL::RENDERBUFFER, depth_buffer.as_ref());

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        renderbuffer
    }

    pub fn create_texture_frame_buffer(
        w: i32,
        h: i32,
        gl: &WebGl2RenderingContext,
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


    // pub fn create_texture_frame_buffer_multisample(
    //     w: i32,
    //     h: i32,
    //     n: u32,
    //     gl: &WebGl2RenderingContext,
    // ) -> (Option<WebGlTexture>, Option<WebGlFramebuffer>) {
    //     let target_texture = gl.create_texture();
    //     gl.active_texture(GL::TEXTURE0);
    //     gl.bind_texture(GL::TEXTURE_2D, target_texture.as_ref());
    //
    //     // define size and format of level 0
    //     let level = 0;
    //     let internal_format = GL::RGBA;
    //     let border = 0;
    //     let format = GL::RGBA;
    //     let gltype = GL::UNSIGNED_BYTE;
    //     gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
    //         GL::TEXTURE_2D,
    //         level,
    //         internal_format as i32,
    //         w,
    //         h,
    //         border,
    //         format,
    //         gltype,
    //         None,
    //     )
    //         .expect("Cannot set texture for FBO");
    //
    //     // set the filtering so we don't need mips
    //     gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
    //     gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
    //     gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
    //
    //     let depth_buffer = gl.create_renderbuffer().unwrap();
    //     gl.bind_renderbuffer(GL::RENDERBUFFER, Some(&depth_buffer));
    //     gl.renderbuffer_storage_multisample(GL::RENDERBUFFER, 4, GL::RBGA8, w, h);
    //     // Create and bind the framebuffer
    //     let fb = gl.create_framebuffer();
    //     gl.bind_framebuffer(GL::FRAMEBUFFER, fb.as_ref());
    //
    //     // attach the texture as the first color attachment
    //     let attachment_point = GL::COLOR_ATTACHMENT0;
    //     gl.framebuffer_texture_2d(
    //         GL::FRAMEBUFFER,
    //         attachment_point,
    //         GL::TEXTURE_2D,
    //         target_texture.as_ref(),
    //         level,
    //     );
    //     gl.framebuffer_renderbuffer(
    //         GL::FRAMEBUFFER,
    //         GL::DEPTH_ATTACHMENT,
    //         GL::RENDERBUFFER,
    //         Some(&depth_buffer),
    //     );
    //
    //     (Some(target_texture.unwrap()), Some(fb.unwrap()))
    // }
}
