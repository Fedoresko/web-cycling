use wasm_bindgen::JsValue;
use web_sys::{WebGlRenderingContext as GL};

use crate::app::ui::picking::PickingRender;
use crate::render::{Render, Vao};
use crate::render::WebRenderer;
use crate::shader::ShaderKind::UIPicking;
use crate::State;

// static BIRD_SPEED: f32 = 3.5;
// static BIRD_START_Z: f32 = -30.0;
// static BIRD_END_Z: f32 = 30.0;

impl WebRenderer {
    pub fn render_picking(&self, gl: &GL, mesh: &impl PickingRender) -> usize {
        self.bind_vao(&Vao(JsValue::NULL.into()));
    
        self.shader_sys.use_program(gl, UIPicking);
        let shader = self.shader_sys.get_shader(&UIPicking).unwrap();
        mesh.render_for_pick(gl, &shader)
    }

    pub fn render_mesh(&self, gl: &GL, state: &State, mesh_name: &str, mesh: &impl Render) {
        //for (mesh_name, mesh) in meshes.into_iter() {
        self.shader_sys.use_program(gl, mesh.shader_kind());

        self.prepare_for_render(gl, mesh, &self.shader_sys, mesh_name);
        let shader = self.shader_sys.get_shader(&mesh.shader_kind()).unwrap();
        mesh.render(gl, state, &shader, self);
        //}

        // Render Bird

        // let skinned_shader = self.shader_sys.get_shader(&skin).unwrap();
        // self.shader_sys.use_program(gl, ShaderKind::SkinnedMesh);
        //
        // let bird_traveled = (state.clock() / 1000.0) * BIRD_SPEED;
        // let z = BIRD_START_Z + (bird_traveled % (BIRD_END_Z - BIRD_START_Z));
        //
        // let mesh_opts = MeshRenderOpts {
        //     pos: (0., 6., z),
        //     clip_plane,
        //     flip_camera_y,
        // };

        // let mesh_name = "Bird";
        // let armature_name = "Armature";
        // let bird = SkinnedMesh {
        //     mesh: assets.get_mesh(mesh_name).expect("Bird mesh"),
        //     armature: assets.get_armature(armature_name).expect("Bird armature"),
        //     shader: skinned_shader,
        //     opts: &mesh_opts,
        // };
        //
        // self.prepare_for_render(gl, &bird, mesh_name);
        // bird.render(gl, state);
    }
}
