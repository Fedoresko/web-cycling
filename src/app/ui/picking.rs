use web_sys::WebGlRenderingContext as GL;

use crate::app::ui::element::Element;
use crate::app::ui::render::RenderableElement;
use crate::render::Render;
use crate::shader::Shader;

pub trait PickingRender {
    fn render_for_pick(&self, gl: &GL, shader: &Shader) -> usize;
}

impl<T> PickingRender for T
    where
        T: RenderableElement,
{
    fn render_for_pick(&self, gl: &GL, shader: &Shader) -> usize {
        self.uniform(gl, shader);
        let color_uni = shader.get_uniform_location(gl, "color");

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let mut t: Vec<f32> = vec![0.5, 0.5, 0.0];
        t.extend(
            self.get_shape()
                .iter()
                .flat_map(|segment| [segment.x, segment.y, 0.0]),
        );
        Element::buffer_f32_data(&gl, &t, pos_attrib as u32, 3);

        gl.uniform1i(color_uni.as_ref(), self.get_id() as i32);
        gl.draw_arrays(GL::TRIANGLE_FAN, 0, (self.get_shape().len() + 1) as i32);
        self.get_id()
    }
}
