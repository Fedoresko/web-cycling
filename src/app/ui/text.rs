use svg_load::font::{Font, Glyph};
use web_sys::{console, WebGlRenderingContext};
use crate::{Render, State, WebRenderer};
use crate::shader::{Shader, ShaderKind};
use web_sys::WebGlRenderingContext as GL;
use crate::geom::Transform;

pub struct RenderableString<'a> {
    pub string: String,
    pub position: (i32, i32),
    pub font_size: f32,
    pub color: [f32; 4],
    pub font: &'a Font,
}

impl Render for RenderableString<'_> {
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::UI
    }

    fn buffer_attributes(&self, _: &GL, _: &Shader) {
    }

    fn render(&self, gl: &GL, state: &State, shader: &Shader, renderer: &WebRenderer) {
        let transform_uni = shader.get_uniform_location(gl, "transform");
        let color_uni = shader.get_uniform_location(gl, "color");
        let blur_uni = shader.get_uniform_location(gl, "blur");
        let texrate_uni = shader.get_uniform_location(gl, "tex_rate");
        let resolution_uni = shader.get_uniform_location(gl, "iResolution");
        let stops_attrib = gl.get_uniform_location(&shader.program, "n_stops");
        let opacity_uni = shader.get_uniform_location(gl, "opacity");

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;

        gl.uniform2fv_with_f32_array(resolution_uni.as_ref(), &[w, h]);
        gl.uniform4fv_with_f32_array(color_uni.as_ref(), &self.color);
        gl.uniform2fv_with_f32_array(
            texrate_uni.as_ref(),
            &[state.width_rate(), state.height_rate()],
        );
        gl.uniform1i(shader.get_uniform_location(gl, "iChannel0").as_ref(), 0);
        gl.uniform1i(blur_uni.as_ref(), 0);
        gl.uniform1i(stops_attrib.as_ref(), 0);
        gl.uniform1f(opacity_uni.as_ref(), 1.0);

        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;
        // let sh_x = -w + 1.0;
        // let sh_y = -h + 1.0;

        let mut pos = (self.position.0 as f32, self.position.1 as f32);
        for char in self.string.chars() {
            if let Some(glyph) = self.font.glyph_map.get(&u32::from(char)) {
                let bb_width = (glyph.bbox.2 - glyph.bbox.0) * self.font_size;
                let scale_x = (bb_width.floor() + 1.0)/bb_width;
                let lower_x = pos.0 + (glyph.bbox.0);
                let trans_x = lower_x.floor() - lower_x;
                let bb_height = (glyph.bbox.3) * self.font_size;
                let scale_y = (bb_height.floor() + 1.0)/bb_height;

                let mut t = Transform::new_translate((pos.0 + trans_x) - w / 2.0, pos.1 + 0.5 - h / 2.0);
                t.scale( self.font_size * scale_x * 2.0 / w, self.font_size * scale_y * 2.0 / h);
                gl.uniform_matrix3fv_with_f32_array(transform_uni.as_ref(), false, &t.to_array());

                // gl.uniform4fv_with_f32_array(
                //     pos_uni.as_ref(),
                //     &[
                //         ((pos.0 + trans_x) * 2.0 + sh_x) / w,
                //         ((pos.1 + 0.5) * 2.0 + sh_y) / h,
                //         self.font_size * scale_x * 2.0 / w,
                //         self.font_size * scale_y * 2.0 / h,
                //     ],
                // );
                pos.0 += glyph.advance*self.font_size;

                renderer.render_mesh(gl,state,&format!("{}{}", &self.font.name,  char), glyph);
            }
        }
    }
}

impl Render for Glyph {
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::UI
    }

    fn buffer_attributes(&self, gl: &WebGlRenderingContext, shader: &Shader) {
        let mesh = &self.outline;
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        console::log_1(&format!("Loading VBO for glyph  v: {} i: {} adv: {}",self.outline.vertices.len(), self.outline.indices.len(), self.advance).into());

        RenderableString::buffer_f32_data(
            &gl,
            &mesh.vertices.iter()
                .flat_map(|v| [v.position[0], v.position[1], 0.0] )
                .collect::<Vec<f32>>(),
            pos_attrib as u32,
            3,
        );

        RenderableString::buffer_u16_indices(&gl, &mesh.indices.iter().map(|i| *i as u16).collect::<Vec<u16>>() );
    }

    fn render(&self, gl: &WebGlRenderingContext, _: &State, _: &Shader, _: &WebRenderer) {
        let num_indices = self.outline.indices.len();
        gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
    }
}
