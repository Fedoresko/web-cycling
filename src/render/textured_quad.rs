use crate::app::State;
use crate::render::Render;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use crate::WebRenderer;

pub struct TexturedQuad {
    /// Left most part of canvas is 0, rightmost is CANVAS_WIDTH
    left: u16,
    /// Bottom of canvas is 0, top is CANVAS_HEIGHT
    top: u16,
    /// How many pixels wide
    width: u16,
    /// How many pixels tall
    height: u16,
    /// Z-index
    depth: f32,
    tex_width: f32,
    tex_height: f32,
    /// The texture unit to use
    texture_unit: u8,
}

impl TexturedQuad {
    pub fn new(
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        depth: f32,
        tex_width: f32,
        tex_height: f32,
        texture_unit: u8,
    ) -> TexturedQuad {
        TexturedQuad {
            left,
            top,
            width,
            height,
            depth,
            tex_width,
            tex_height,
            texture_unit,
        }
    }
}

impl Render for TexturedQuad {
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::TexturedQuad
    }

    fn buffer_attributes(&self, gl: &GL, shader: &Shader) {
        let vertex_data =
            self.make_textured_quad_vertices(gl.drawing_buffer_width(), gl.drawing_buffer_height());

        let vertex_data_attrib = gl.get_attrib_location(&shader.program, "vertexData");
        gl.enable_vertex_attrib_array(vertex_data_attrib as u32);

        TexturedQuad::buffer_f32_data(&gl, &vertex_data[..], vertex_data_attrib as u32, 4);
    }

    fn render(&self, gl: &WebGlRenderingContext, _state: &State, shader: &Shader, _: &WebRenderer) {
        gl.uniform1i(
            shader.get_uniform_location(gl, "texture").as_ref(),
            self.texture_unit as i32,
        );

        gl.uniform1f(
            shader.get_uniform_location(gl, "depth").as_ref(),
            self.depth,
        );
        gl.uniform2fv_with_f32_array(
            shader.get_uniform_location(gl, "texrate").as_ref(),
            &[self.tex_width, self.tex_height],
        );
        // gl.uniform2fv_with_f32_array(
        //     shader.get_uniform_location(gl, "resolution").as_ref(),
        //     &[self.width as f32, self.height as f32],
        // );

        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}

impl TexturedQuad {
    // Combine our vertex data so that we can pass one array to the GPU
    fn make_textured_quad_vertices(&self, viewport_width: i32, viewport_height: i32) -> Vec<f32> {
        let viewport_width = viewport_width as f32;
        let viewport_height = viewport_height as f32;

        let left_x = self.left as f32 / viewport_width;
        let top_y = self.top as f32 / viewport_height;
        let right_x = (self.left as f32 + self.width as f32) / viewport_width;
        let bottom_y = (self.top as f32 + self.height as f32) / viewport_height;

        let left_x = 2.0 * left_x - 1.0;
        let right_x = 2.0 * right_x - 1.0;

        let bottom_y = 2.0 * bottom_y - 1.0;
        let top_y = 2.0 * top_y - 1.0;

        // All of the positions of our quad in screen space
        let positions = [
            left_x, top_y, // Top Left
            right_x, bottom_y, // Bottom Right
            left_x, bottom_y, // Bottom Left
            left_x, top_y, // Top Left
            right_x, top_y, // Top Right
            right_x, bottom_y, // Bottom Right
        ];

        let texture_coords = [
            0., 0., // Top left
            1.0, 1.0, // Bottom Right
            0., 1.0, // Bottom Left
            0., 0., // Top Left
            1.0, 0.0, // Top Right
            1.0, 1.0, // Bottom Right
        ];

        let mut vertices = vec![];

        for i in 0..positions.len() {
            // Skip odd indices
            if i % 2 == 1 {
                continue;
            }

            vertices.push(positions[i]);
            vertices.push(positions[i + 1]);
            vertices.push(texture_coords[i]);
            vertices.push(texture_coords[i + 1]);
        }

        vertices
    }
}
