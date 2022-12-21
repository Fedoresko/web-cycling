use web_sys::WebGl2RenderingContext as GL;

use crate::app::ui::element::Element;
use crate::app::ui::render::RenderableElement;
use crate::geom::Transform;
use crate::render::Render;
use crate::shader::Shader;
use crate::WebRenderer;

pub trait PickingRender {
    fn render_for_pick(&self, gl: &GL, shader: &Shader, renderer: &WebRenderer) -> usize;
}

impl<T> PickingRender for T
    where
        T: RenderableElement,
{
    fn render_for_pick(&self, gl: &GL, shader: &Shader, renderer: &WebRenderer) -> usize {
        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;

        let color_uni = shader.get_uniform_location(gl, "color");
        let id = self.get_id();
        gl.uniform1i(color_uni.as_ref(), id as i32);

        if let Some(svg) = self.get_svg() {
            let transform_uni = shader.get_uniform_location(gl, "transform");
            let pos = self.get_position();
            let sz = self.get_size();

            let mut t = Transform::new_translate(  2.0 * pos.0 as f32 / w - 1.0 + 1.0/w, 2.0 * pos.1 as f32 / h - 1.0  + 1.0/h);
            t.scale( sz.0 as f32 * 2.0 / w, sz.1 as f32 * 2.0 / h);
            gl.uniform_matrix3fv_with_f32_array(transform_uni.as_ref(), false, &t.to_array());

            for (k, mesh) in renderer.get_assets().get_image(svg.as_str()).unwrap().iter().rev().enumerate() {
                renderer.render_picking(gl, mesh);
            }
        }

        self.uniform(gl, shader);

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let mut t: Vec<f32> = vec![0.5, 0.5, 0.0];
        t.extend(
            self.get_shape()
                .iter()
                .flat_map(|segment| [segment.x, segment.y, 0.0]),
        );
        Element::buffer_f32_data(&gl, &t, pos_attrib as u32, 3);

        gl.draw_arrays(GL::TRIANGLE_FAN, 0, (self.get_shape().len() + 1) as i32);
        id
    }
}
