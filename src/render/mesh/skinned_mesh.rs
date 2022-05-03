use blender_armature::{BlenderArmature, Bone, FrameOffset, JointIndicesRef};
use blender_armature::SampleDesc;
use blender_mesh::SingleIndexedVertexAttributes;
use nalgebra;
use nalgebra::{Isometry3, Vector3};
use wasm_bindgen::__rt::core::time::Duration;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

use crate::app::State;
use crate::render::mesh::non_skinned_mesh::MeshRenderOpts;
use crate::render::Render;
use crate::render::TextureUnit;
use crate::shader::Shader;
use crate::shader::ShaderKind;

pub struct SkinnedMesh<'a> {
    pub mesh: &'a SingleIndexedVertexAttributes,
    pub armature: &'a BlenderArmature,
    pub opts: &'a MeshRenderOpts,
}

impl Render for SkinnedMesh<'_> {
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::SkinnedMesh
    }

    fn buffer_attributes(&self, gl: &WebGlRenderingContext, shader: &Shader) {
        let mesh = self.mesh;

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
        let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

        gl.enable_vertex_attrib_array(pos_attrib as u32);
        gl.enable_vertex_attrib_array(normal_attrib as u32);
        gl.enable_vertex_attrib_array(uv_attrib as u32);

        SkinnedMesh::buffer_f32_data(
            &gl,
            &mesh
                .vertices()
                .into_iter()
                .flat_map(|v| v.position())
                .collect::<Vec<f32>>(),
            pos_attrib as u32,
            3,
        );
        SkinnedMesh::buffer_f32_data(
            &gl,
            &mesh
                .vertices()
                .into_iter()
                .flat_map(|v| v.normal().unwrap())
                .collect::<Vec<f32>>(),
            normal_attrib as u32,
            3,
        );
        SkinnedMesh::buffer_f32_data(
            &gl,
            &mesh
                .vertices()
                .into_iter()
                .flat_map(|v| v.uv().unwrap())
                .collect::<Vec<f32>>(), // &mesh.vertex_uvs.as_ref().expect("Mesh uvs")[..],
            uv_attrib as u32,
            2,
        );
        SkinnedMesh::buffer_u16_indices(&gl, mesh.indices());
    }

    fn render(&self, gl: &WebGlRenderingContext, state: &State, shader: &Shader) {
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
        gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view);

        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.to_homogeneous().as_slice());
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        let mut perspective = state.camera().projection();
        gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut perspective);

        let camera_pos = state.camera().get_eye_pos();
        let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
        gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &mut camera_pos);

        gl.uniform1i(mesh_texture_uni.as_ref(), TextureUnit::Stad.texture_unit());

        self.set_armature_uniforms(gl, state, shader);

        let num_indices = mesh.vertices().len();
        gl.draw_elements_with_i32(GL::TRIANGLES, num_indices as i32, GL::UNSIGNED_SHORT, 0);
    }
}

impl<'a> SkinnedMesh<'a> {
    fn set_armature_uniforms(&self, gl: &WebGlRenderingContext, state: &State, shader: &Shader) {
        let armature = &self.armature;

        let clock = state.clock();
        let current_time_secs = Duration::from_secs_f32(clock / 1000.0);

        let bone_indexes = armature
            .joint_indices()
            .values()
            .map(|v| v.clone())
            .collect::<Vec<u8>>(); //&[0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]

        console::log_1(
            &format!("Actions {:?}", armature.bone_space_actions().keys()).into(),
        );

        let bones = armature.interpolate_bones(
            "Fly",
            JointIndicesRef::Some(&bone_indexes),
            SampleDesc {
                frame_offset: FrameOffset::new_with_elapsed_time_and_frames_per_second(
                    current_time_secs,
                    24,
                ),
                should_loop: true,
            },
        );
        console::log_1(&format!("time {}", current_time_secs.as_secs_f32()).into());

        let bone_count = bones.len() as u8;

        for index in 0..bone_count {
            let bone = bones.get(&index).expect("Interpolated bone");

            let dual_quat = BlenderArmature::matrix_to_dual_quat(bone);

            match dual_quat {
                Bone::DualQuat(q) => {
                    if index == 0 {
                        console::log_1(
                            &format!(
                                "boneRotQuaternions[{}] [{},{},{},{}]",
                                index,
                                q.real.coords.x,
                                q.real.coords.y,
                                q.real.coords.x,
                                q.real.coords.w
                            )
                                .into(),
                        );
                    }
                    let rot_quat_uni =
                        shader.get_uniform_location(gl, &format!("boneRotQuaternions[{}]", index));
                    gl.uniform4f(
                        rot_quat_uni.as_ref(),
                        q.real.coords.x,
                        q.real.coords.y,
                        q.real.coords.z,
                        q.real.coords.w,
                    );

                    let trans_quat_uni = shader
                        .get_uniform_location(gl, &format!("boneTransQuaternions[{}]", index));
                    gl.uniform4f(
                        trans_quat_uni.as_ref(),
                        q.dual.coords.x,
                        q.dual.coords.y,
                        q.dual.coords.z,
                        q.dual.coords.w,
                    );
                }
                Bone::Matrix(_m) => (),
            }
        }
    }
}
