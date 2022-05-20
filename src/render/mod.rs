use std::cell::RefCell;
use std::collections::HashMap;

use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;

use crate::app::Assets;
use crate::app::State;
use crate::shader::ShaderSystem;

pub(self) use self::mesh::*;
pub use self::render_trait::*;
pub use self::texture_unit::*;

pub mod framebuffer;
mod mesh;
mod render_meshes;
mod render_trait;
mod texture_unit;
pub mod textured_quad;

struct VaoExtension {
    vaos: RefCell<HashMap<String, Vao>>,
}

pub type Vao = Option<WebGlVertexArrayObject>;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    vao_ext: VaoExtension,
    assets: Assets,
}

impl WebRenderer {
    pub fn new(gl: &WebGl2RenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        let vao_ext = VaoExtension {
            vaos: RefCell::new(HashMap::new()),
        };

        WebRenderer {
            shader_sys,
            vao_ext,
            assets: Assets::new(),
        }
    }

    pub fn render(&self, gl: &WebGl2RenderingContext, state: &State) {
        gl.clear_color(0.5, 0.5, 0.5, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.enable(GL::DEPTH_TEST);

        gl.viewport(0, 0, state.viewport_width(), state.viewport_height());

        if !state.show_scenery() {
            return;
        }


        // let mut meshes : HashMap<&str, &dyn Render> = HashMap::new();
        let mesh_opts = MeshRenderOpts {
            pos: (0.0, 0.0, 0.0),
            flip_camera_y: false,
        };

        let mesh_name = "Goose";
        let sphere = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("Goose mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Goose mesh"),
            texture: None,
        };

        // let mesh_opts = MeshRenderOpts {
        //     pos: (3.0, 2.0, 0.0),
        //     flip_camera_y: false,
        // };
        self.render_mesh(gl, state, mesh_name, &sphere);
        let mesh_name = "BMXBody";
        let sphere = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("BMX Body mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Goose mesh"),
            texture: None,
        };
        self.render_mesh(gl, state, mesh_name, &sphere);
        let mesh_name = "BMXHandle";
        let sphere = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("BMX Handle mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Goose mesh"),
            texture: None,
        };
        self.render_mesh(gl, state, mesh_name, &sphere);
        let mesh_name = "BMXFrontWheel";
        let sphere = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("BMX Front Wheel mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Goose mesh"),
            texture: None,
        };
        self.render_mesh(gl, state, mesh_name, &sphere);
        let mesh_name = "BMXRearWheel";
        let sphere = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("BMX Rear Wheel mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Goose mesh"),
            texture: None,
        };
        self.render_mesh(gl, state, mesh_name, &sphere);
        let mesh_name = "BMXPedals";
        let sphere = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("BMX Pedals mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Goose mesh"),
            texture: None,
        };
        self.render_mesh(gl, state, mesh_name, &sphere);


        // let mesh_opts = MeshRenderOpts {
        //     pos: (0., 10., 50.),
        //     flip_camera_y: false,
        // };


        let mesh_name = "Sphere";
        let sphere = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("Sphere mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Sphere mesh"),
            texture: Some(&TextureUnit::Stad),
        };
        self.render_mesh(gl, state, mesh_name, &sphere);

        // let mut meshes : HashMap<&str, &dyn Render> = HashMap::new();
        // let mesh_opts = MeshRenderOpts {
        //     pos: (0., 10., -10.),
        //     flip_camera_y: false,
        // };

        let mesh_name = "Velodrome";
        let velodrome = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("Velodrome mesh"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Velodrome mesh"),
            texture: Some(&TextureUnit::Velodrome),
        };
        self.render_mesh(gl, state, mesh_name, &velodrome);

        let mesh_name = "VelodromeFlat";
        let velodrome = NonSkinnedMesh {
            mesh: self.assets.get_mesh(mesh_name).expect("Velodrome flat"),
            opts: &mesh_opts,
            full_mesh: self.assets.get_full_mesh(mesh_name).expect("Velodrome flat"),
            texture: Some(&TextureUnit::VelodromeFlat),
        };
        self.render_mesh(gl, state, mesh_name, &velodrome);
    }

    // fn create_vao(&self) -> Vao {
    //     let oes_vao_ext = &self.vao_ext.oes_vao_ext;
    //
    //     let create_vao_ext = Reflect::get(oes_vao_ext, &"createVertexArrayOES".into())
    //         .expect("Create vao func")
    //         .into();
    //
    //     Vao(
    //         Reflect::apply(&create_vao_ext, oes_vao_ext, &js_sys::Array::new())
    //             .expect("Created vao")
    //             .into(),
    //     )
    // }

    pub fn get_assets(&self) -> &Assets {
        &self.assets
    }

    fn prepare_for_render<'a>(
        &self,
        gl: &WebGl2RenderingContext,
        renderable: &impl Render,
        shader_system: &ShaderSystem,
        key: &str,
    ) {
        if self.vao_ext.vaos.borrow().get(key).is_none() {
            let vao = gl.create_vertex_array();
            gl.bind_vertex_array(vao.as_ref());
            let shader = shader_system.get_shader(&renderable.shader_kind()).unwrap();
            renderable.buffer_attributes(gl, &shader);
            self.vao_ext.vaos.borrow_mut().insert(key.to_string(), vao);
            return;
        }

        let vaos = self.vao_ext.vaos.borrow();
        let vao = vaos.get(key).unwrap();
        gl.bind_vertex_array(vao.as_ref());
    }

    // fn bind_vao(&self, vao: &Vao) {
    //     let oes_vao_ext = &self.vao_ext.oes_vao_ext;
    //
    //     let bind_vao_ext = Reflect::get(&oes_vao_ext, &"bindVertexArrayOES".into())
    //         .expect("Create vao func")
    //         .into();
    //
    //     let args = js_sys::Array::new();
    //     args.push(&vao.0);
    //
    //     Reflect::apply(&bind_vao_ext, oes_vao_ext, &args).expect("Bound VAO");
    // }
}
