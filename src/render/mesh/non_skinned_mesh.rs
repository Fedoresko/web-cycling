use blender_mesh::SingleIndexedVertexAttributes;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Vector3};
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

use crate::app::State;
use crate::render::Render;
use crate::render::TextureUnit;
use crate::shader::Shader;
use crate::shader::ShaderKind;
use crate::WebRenderer;

pub struct NonSkinnedMesh<'a> {
    pub mesh: &'a SingleIndexedVertexAttributes,
    pub opts: &'a MeshRenderOpts,
    pub texture: &'a TextureUnit,
}

pub struct MeshRenderOpts {
    pub pos: (f32, f32, f32),
    pub flip_camera_y: bool,
}

impl Render for NonSkinnedMesh<'_> {
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::NonSkinnedMesh
    }

    fn buffer_attributes(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        let mesh = self.mesh;

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
        let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

        gl.enable_vertex_attrib_array(pos_attrib as u32);
        gl.enable_vertex_attrib_array(normal_attrib as u32);
        gl.enable_vertex_attrib_array(uv_attrib as u32);

        //let verices = mesh.combine_vertex_indices(&CreateSingleIndexConfig { bone_influences_per_vertex: None, calculate_face_tangents: false });
        // mesh.multi_indexed_vertex_attributes.

        NonSkinnedMesh::buffer_f32_data(
            &gl,
            &mesh
                .vertices()
                .into_iter()
                .flat_map(|v| v.position())
                .collect::<Vec<f32>>(),
            pos_attrib as u32,
            3,
        );
        NonSkinnedMesh::buffer_f32_data(
            &gl,
            &mesh
                .vertices()
                .into_iter()
                .flat_map(|v| v.normal().unwrap())
                .collect::<Vec<f32>>(),
            normal_attrib as u32,
            3,
        );
        NonSkinnedMesh::buffer_f32_data(
            &gl,
            &mesh
                .vertices()
                .into_iter()
                .flat_map(|v| v.uv().unwrap())
                .collect::<Vec<f32>>(), // &mesh.vertex_uvs.as_ref().expect("Mesh uvs")[..],
            uv_attrib as u32,
            2,
        );
        NonSkinnedMesh::buffer_u16_indices(&gl, mesh.indices());
    }

    fn render(&self, gl: &WebGl2RenderingContext, state: &State, shader: &Shader, _:&WebRenderer) {
        let mesh = self.mesh;
        let opts = self.opts;
        let pos = opts.pos;

        let model_uni = shader.get_uniform_location(gl, "model");
        let view_uni = shader.get_uniform_location(gl, "view");
        let camera_pos_uni = shader.get_uniform_location(gl, "cameraPos");
        let perspective_uni = shader.get_uniform_location(gl, "perspective");
        //let clip_plane_uni = shader.get_uniform_location(gl, "clipPlane");
        let mesh_texture_uni = shader.get_uniform_location(gl, "meshTexture");

        //gl.uniform4fv_with_f32_array(clip_plane_uni.as_ref(), &mut opts.clip_plane.clone()[..]);

        let mut view = if opts.flip_camera_y {
            state.camera().view_flipped_y()
        } else {
            state.camera().view()
        };

        // var modelviewInv = new Float32Array(16);
        // var normalmatrix = new Float32Array(16);
        let mat: Matrix4<f32> = Matrix4::from_iterator((&view).iter().map(|e| *e));
        let mat_inv = mat.try_inverse().unwrap().transpose();
        let norm_mat_uni = shader.get_uniform_location(gl, "normalMat");
        let mut norm_mat = [0.; 16];
        norm_mat.copy_from_slice(mat_inv.as_slice());
        gl.uniform_matrix4fv_with_f32_array(norm_mat_uni.as_ref(), false, &mut norm_mat);

        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view);

        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.to_homogeneous().as_slice());
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        let camera_pos = state.camera().get_eye_pos();
        let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
        gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &mut camera_pos);

        gl.uniform1i(mesh_texture_uni.as_ref(), self.texture.texture_unit());

        let mut perspective = state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut perspective);

        let num_indices = mesh.vertices().len();
        gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
    }
}
