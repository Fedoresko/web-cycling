use crate::{Render, State, WebRenderer};
use crate::shader::{Shader, ShaderKind};
use web_sys::{console, WebGlRenderingContext as GL};
use svg_load::path::RenderablePath;

impl Render for RenderablePath {
    fn shader_kind(&self) -> ShaderKind { ShaderKind::UI }

    fn buffer_attributes(&self, gl: &GL, shader: &Shader) {
        let mesh = &self.vertices;
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        RenderablePath::buffer_f32_data(
            &gl,
            &mesh.vertices.iter()
                .flat_map(|v| [v.position[0], v.position[1], 0.0] )
                .collect::<Vec<f32>>(),
            pos_attrib as u32,
            3,
        );

        RenderablePath::buffer_u16_indices(&gl, &mesh.indices.iter().map(|i| *i as u16).collect::<Vec<u16>>() );
    }

    fn render(&self, gl: &GL, state: &State, shader: &Shader, _: &WebRenderer) {
        let pos_uni = shader.get_uniform_location(gl, "element_pos");
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        let pos = (0,0);
        let sz = self.size;

        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;
        let sh_x = -w + 1.0;
        let sh_y = -h + 1.0;

        gl.uniform4fv_with_f32_array(
            pos_uni.as_ref(),
            &[
                ((pos.0 * 2) as f32 + sh_x) / w,
                ((pos.1 * 2) as f32 + sh_y) / h,
                (sz.0 * 2) as f32 / w,
                (sz.1 * 2) as f32 / h,
            ],
        );

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
        gl.uniform4fv_with_f32_array(color_uni.as_ref(), &self.bgcolor);
        gl.uniform2fv_with_f32_array(
            texrate_uni.as_ref(),
            &[state.width_rate(), state.height_rate()],
        );
        gl.uniform1i(shader.get_uniform_location(gl, "iChannel0").as_ref(), 0);

        gl.uniform1i(blur_uni.as_ref(), 0);

        gl.uniform1i(stops_attrib.as_ref(), self.gradient_stops as i32);
        if self.gradient_stops > 0 {
            let x : Vec<f32> = self.gradient_colors.as_ref().unwrap().iter().flatten().map(|a| *a).collect();
            gl.uniform4fv_with_f32_array(stops_color_attrib.as_ref(), x.as_slice());
            gl.uniform1fv_with_f32_array(stops_positions_attrib.as_ref(), self.gradient_pos.as_ref().unwrap() );
            let start = self.gradient_start.unwrap();
            let end = self.gradient_end.unwrap();
            let grad_coords = [start.0, start.1, end.0, end.1];
            gl.uniform2fv_with_f32_array(grad_coords_attrib.as_ref(), &grad_coords);
        }

        let num_indices = self.vertices.indices.len();
        gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
    }
}

