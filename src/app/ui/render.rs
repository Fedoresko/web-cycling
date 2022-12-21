use web_sys::{WebGl2RenderingContext};
use web_sys::WebGl2RenderingContext as GL;

use crate::app::ui::element::{Element, LineStyle, ShapeSegment};
use crate::fields::Vec4;
use crate::render::Render;
use crate::shader::{Shader, ShaderKind};
use crate::{State, WebRenderer};
use crate::geom::Transform;
use crate::text::RenderableString;

pub trait RenderableElement {
    fn get_id(&self) -> usize;
    fn get_shape(&self) -> &[ShapeSegment];
    fn get_style(&self) -> Option<&LineStyle>;
    fn is_blur(&self) -> bool;
    fn get_bg_color(&self) -> [f32; 4];
    fn get_position(&self) -> (i32, i32);
    fn get_size(&self) -> (i32, i32);
    fn get_gradient_stops_n(&self) -> u8;
    fn get_gradient_positions(&self) -> &[f32];
    fn get_gradient_colors(&self) -> &[Vec4];
    fn get_gradient_start(&self) -> (f32, f32);
    fn get_gradient_end(&self) -> (f32, f32);
    fn get_svg(&self) -> &Option<String>;
    fn get_label(&self) -> &Option<RenderableString>;
    fn get_opacity(&self) -> f32;

    fn uniform(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        let transform_uni = shader.get_uniform_location(gl, "transform");
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;
        let pos = self.get_position();
        let sz = self.get_size();

        let mut t = Transform::new_translate(  2.0 * pos.0 as f32 / w - 1.0 + 1.0/w, 2.0 * pos.1 as f32 / h - 1.0  + 1.0/h);
        t.scale( sz.0 as f32 * 2.0 / w, sz.1 as f32 * 2.0 / h);

        gl.uniform_matrix3fv_with_f32_array(transform_uni.as_ref(), false, &t.to_array());
    }
}

impl<T> Render for T
where
    T: RenderableElement,
{
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::UI
    }

    fn buffer_attributes(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        self.uniform(gl, shader);

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let t: Vec<f32> = self
            .get_shape()
            .iter()
            .flat_map(|segment| [segment.x, segment.y, 0.0])
            .collect();
        Element::buffer_f32_data(&gl, &t, pos_attrib as u32, 3);
    }

    fn render(&self, gl: &WebGl2RenderingContext, state: &State, shader: &Shader, renderer: &WebRenderer) {
        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;

        if let Some(label) = self.get_label() {
            let opacity_uni = shader.get_uniform_location(gl, "opacity");
            let pos = self.get_position();
            label.pos.set(pos.clone());

            gl.uniform1f(opacity_uni.as_ref(), self.get_opacity());
            renderer.render_mesh(gl, state, &format!("label{}",self.get_id()), label);
        }

        if let Some(svg) = self.get_svg() {
            let transform_uni = shader.get_uniform_location(gl, "transform");
            let opacity_uni = shader.get_uniform_location(gl, "opacity");
            let pos = self.get_position();
            let sz = self.get_size();

            let mut t = Transform::new_translate(  2.0 * pos.0 as f32 / w - 1.0 + 1.0/w, 2.0 * pos.1 as f32 / h - 1.0  + 1.0/h);
            t.scale( sz.0 as f32 * 2.0 / w, sz.1 as f32 * 2.0 / h);
            gl.uniform_matrix3fv_with_f32_array(transform_uni.as_ref(), false, &t.to_array());
            gl.uniform1f(opacity_uni.as_ref(), self.get_opacity());

            for (k, mesh) in renderer.get_assets().get_image(svg.as_str()).unwrap().iter().rev().enumerate() {
                renderer.render_mesh(gl, state, &format!("svg{}_{}",self.get_id(),k), mesh);
            }
        }

        if self.get_shape().len() > 0 {
            self.buffer_attributes(gl, shader);

            let opacity_uni = shader.get_uniform_location(gl, "opacity");
            let color_uni = shader.get_uniform_location(gl, "color");
            let blur_uni = shader.get_uniform_location(gl, "blur");
            let texrate_uni = shader.get_uniform_location(gl, "tex_rate");
            let resolution_uni = shader.get_uniform_location(gl, "iResolution");

            let stops_attrib = gl.get_uniform_location(&shader.program, "n_stops");
            let stops_color_attrib = gl.get_uniform_location(&shader.program, "color_stops");
            let stops_positions_attrib = gl.get_uniform_location(&shader.program, "stop_pos");
            let grad_coords_attrib = gl.get_uniform_location(&shader.program, "gradient_pts");

            gl.uniform2fv_with_f32_array(
                texrate_uni.as_ref(),
                &[state.width_rate(), state.height_rate()],
            );
            gl.uniform2fv_with_f32_array(resolution_uni.as_ref(), &[w, h]);
            gl.uniform1i(shader.get_uniform_location(gl, "iChannel0").as_ref(), 0);
            gl.uniform1f(opacity_uni.as_ref(), self.get_opacity());

            if let Some(line_style) = self.get_style() {
                gl.uniform4fv_with_f32_array(color_uni.as_ref(), &line_style.color);
                gl.uniform1i(stops_attrib.as_ref(), 0);
                gl.uniform1i(blur_uni.as_ref(), 0);

                gl.draw_arrays(GL::LINE_STRIP, 0, self.get_shape().len() as i32);
            }

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
                // console::log_1(&format!("gradient stops {}; start{}; end{}", self.get_gradient_stops_n(), self.get_gradient_start().0, self.get_gradient_end().0).into());
                let x: Vec<f32> = self.get_gradient_colors().iter().flatten().map(|a| *a).collect();
                gl.uniform4fv_with_f32_array(stops_color_attrib.as_ref(), x.as_slice());
                gl.uniform1fv_with_f32_array(stops_positions_attrib.as_ref(), self.get_gradient_positions());
                let grad_coords = [self.get_gradient_start().0, self.get_gradient_start().1,
                    self.get_gradient_end().0, self.get_gradient_end().1];
                gl.uniform2fv_with_f32_array(grad_coords_attrib.as_ref(), &grad_coords);
            }

            gl.draw_arrays(GL::TRIANGLE_FAN, 0, (self.get_shape().len() + 1) as i32);
        }
    }
}
