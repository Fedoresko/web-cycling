use crate::{Render, State, WebRenderer};
use crate::shader::{Shader, ShaderKind};
use web_sys::{WebGl2RenderingContext as GL};
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
        //let transform_uni = shader.get_uniform_location(gl, "transform");
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        // let w = gl.drawing_buffer_width() as f32;
        // let h = gl.drawing_buffer_height() as f32;
        // let sh_x = -w + 1.0;
        // let sh_y = -h + 1.0;

        // gl.uniform4fv_with_f32_array(
        //     pos_uni.as_ref(),
        //     &[
        //         ((self.pos.0 * 2) as f32 + sh_x) / w,
        //         ((self.pos.1 * 2) as f32 + sh_y) / h,
        //         (self.sz.0 * 2) as f32 / w,
        //         (self.sz.1 * 2) as f32 / h,
        //     ],
        // );

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
        let color = [self.bgcolor[0], self.bgcolor[1], self.bgcolor[2], self.bgcolor[3]];
        gl.uniform4fv_with_f32_array(color_uni.as_ref(), &color);
        gl.uniform2fv_with_f32_array(
            texrate_uni.as_ref(),
            &[state.width_rate(), state.height_rate()],
        );
        gl.uniform1i(shader.get_uniform_location(gl, "iChannel0").as_ref(), 0);

        gl.uniform1i(blur_uni.as_ref(), 0);

        gl.uniform1i(stops_attrib.as_ref(), self.gradient_stops as i32);
        if self.gradient_stops > 0 {
            let x : Vec<f32> = self.gradient_colors.as_ref().unwrap().iter()
                .map(move |col| [col[0], col[1], col[2], col[3]] ).flatten().collect();
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

