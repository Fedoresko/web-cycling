use std::fmt::{Display, Formatter};
use web_sys::{console, WebGlRenderingContext};
use web_sys::WebGlRenderingContext as GL;

use crate::app::ui::element::{Element, LineStyle, ShapeSegment};
use crate::render::Render;
use crate::shader::{Shader, ShaderKind};
use crate::State;

pub trait RenderableElement {
    fn get_id(&self) -> u32;
    fn get_shape(&self) -> &[ShapeSegment];
    fn get_style(&self) -> &LineStyle;
    fn is_blur(&self) -> bool;
    fn get_bg_color(&self) -> [f32; 4];
    fn get_position(&self) -> (i32, i32);
    fn get_size(&self) -> (u32, u32);
    fn get_gradient_stops_n(&self) -> u8;
    fn get_gradient_positions(&self) -> &[f32];
    fn get_gradient_colors(&self) -> &[[f32; 4]];
    fn get_gradient_start(&self) -> (f32, f32);
    fn get_gradient_end(&self) -> (f32, f32);

    fn uniform(&self, gl: &WebGlRenderingContext, shader: &Shader) {
        let pos_uni = shader.get_uniform_location(gl, "element_pos");
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;
        let sh_x = -w + 1.0;
        let sh_y = -h + 1.0;

        let pos = self.get_position();
        let sz = self.get_size();
        gl.uniform4fv_with_f32_array(
            pos_uni.as_ref(),
            &[
                ((pos.0 * 2) as f32 + sh_x) / w,
                ((pos.1 * 2) as f32 + sh_y) / h,
                (sz.0 * 2) as f32 / w,
                (sz.1 * 2) as f32 / h,
            ],
        );
    }
}

impl<T> Render for T
where
    T: RenderableElement,
{
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::UI
    }

    fn buffer_attributes(&self, gl: &WebGlRenderingContext, shader: &Shader) {
        self.uniform(gl, shader);

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let t: Vec<f32> = self
            .get_shape()
            .iter()
            .flat_map(|segment| [segment.x, segment.y, 0.0])
            .collect();
        Element::buffer_f32_data(&gl, &t, pos_attrib as u32, 3);
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, shader: &Shader) {
        self.buffer_attributes(gl, shader);
        let color_uni = shader.get_uniform_location(gl, "color");
        let blur_uni = shader.get_uniform_location(gl, "blur");
        let texrate_uni = shader.get_uniform_location(gl, "tex_rate");
        let resolution_uni = shader.get_uniform_location(gl, "iResolution");

        let stops_attrib = gl.get_uniform_location(&shader.program, "n_stops");
        let stops_color_attrib = gl.get_uniform_location(&shader.program, "color_stops");
        let stops_positions_attrib = gl.get_uniform_location(&shader.program, "stop_pos");
        let grad_coords_attrib = gl.get_uniform_location(&shader.program, "gradient_pts");

        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;

        gl.uniform2fv_with_f32_array(resolution_uni.as_ref(), &[w, h]);
        gl.uniform4fv_with_f32_array(color_uni.as_ref(), &self.get_style().color);
        gl.uniform1i(stops_attrib.as_ref(), 0);
        gl.uniform2fv_with_f32_array(
            texrate_uni.as_ref(),
            &[state.width_rate(), state.height_rate()],
        );
        gl.uniform1i(shader.get_uniform_location(gl, "iChannel0").as_ref(), 0);
        gl.uniform1i(blur_uni.as_ref(), 0);

        gl.draw_arrays(GL::LINE_STRIP, 0, self.get_shape().len() as i32);

        //self.uniform(gl, shader);

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let mut t: Vec<f32> = vec![0.5, 0.5, 0.0];
        t.extend(
            self.get_shape()
                .iter()
                .flat_map(|segment| [segment.x, segment.y, 0.0]),
        );
        Element::buffer_f32_data(&gl, &t, pos_attrib as u32, 3);

        gl.uniform1i(blur_uni.as_ref(), if self.is_blur() { 1 } else { 0 });
        gl.uniform4fv_with_f32_array(color_uni.as_ref(), &self.get_bg_color());

        gl.uniform1i(stops_attrib.as_ref(), self.get_gradient_stops_n() as i32);
        if self.get_gradient_stops_n() > 0 {
            console::log_1(&format!("gradient stops {}; start{}; end{}", self.get_gradient_stops_n(), self.get_gradient_start().0, self.get_gradient_end().0).into());
            let x : Vec<f32> = self.get_gradient_colors().iter().flatten().map(|a| *a).collect();
            gl.uniform4fv_with_f32_array(stops_color_attrib.as_ref(), x.as_slice());
            gl.uniform1fv_with_f32_array(stops_positions_attrib.as_ref(), self.get_gradient_positions());
            let grad_coords = [self.get_gradient_start().0, self.get_gradient_start().1,
                self.get_gradient_end().0, self.get_gradient_end().1];
            gl.uniform2fv_with_f32_array(grad_coords_attrib.as_ref(), &grad_coords);
        }

        gl.draw_arrays(GL::TRIANGLE_FAN, 0, (self.get_shape().len() + 1) as i32);
    }
}
