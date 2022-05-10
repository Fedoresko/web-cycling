use std::cell::RefCell;
use std::collections::HashMap;

use js_sys::Reflect;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

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
    oes_vao_ext: js_sys::Object,
    vaos: RefCell<HashMap<String, Vao>>,
}

pub struct Vao(js_sys::Object);

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    #[allow(unused)]
    depth_texture_ext: Option<js_sys::Object>,
    vao_ext: VaoExtension,
}

impl WebRenderer {
    pub fn new(gl: &WebGlRenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        let depth_texture_ext = gl
            .get_extension("WEBGL_depth_texture")
            .expect("Depth texture extension");

        let oes_vao_ext = gl
            .get_extension("OES_vertex_array_object")
            .expect("Get OES vao ext")
            .expect("OES vao ext");

        let vao_ext = VaoExtension {
            oes_vao_ext,
            vaos: RefCell::new(HashMap::new()),
        };

        WebRenderer {
            depth_texture_ext,
            shader_sys,
            vao_ext,
        }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, state: &State, assets: &Assets) {
        gl.clear_color(0.5, 0.5, 0.5, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.enable(GL::DEPTH_TEST);

        gl.viewport(0, 0, state.viewport_width(), state.viewport_height());

        if !state.show_scenery() {
            return;
        }

        // let mut meshes : HashMap<&str, &dyn Render> = HashMap::new();
        let mesh_opts = MeshRenderOpts {
            pos: (0., 10., -10.),
            flip_camera_y: false,
        };

        // let mesh_name = "Sphere";
        // let sphere = NonSkinnedMesh {
        //     mesh: assets.get_mesh(mesh_name).expect("Sphere mesh"),
        //     opts: &mesh_opts,
        //     texture: &TextureUnit::Stone,
        // };
        // self.render_mesh(gl, state, mesh_name, &sphere);

        let mesh_name = "Velodrome";
        let velodrome = NonSkinnedMesh {
            mesh: assets.get_mesh(mesh_name).expect("Velodrome mesh"),
            opts: &mesh_opts,
            texture: &TextureUnit::Velodrome,
        };
        self.render_mesh(gl, state, mesh_name, &velodrome);
    }

    fn create_vao(&self) -> Vao {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let create_vao_ext = Reflect::get(oes_vao_ext, &"createVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        Vao(
            Reflect::apply(&create_vao_ext, oes_vao_ext, &js_sys::Array::new())
                .expect("Created vao")
                .into(),
        )
    }

    fn prepare_for_render<'a>(
        &self,
        gl: &WebGlRenderingContext,
        renderable: &impl Render,
        shader_system: &ShaderSystem,
        key: &str,
    ) {
        if self.vao_ext.vaos.borrow().get(key).is_none() {
            let vao = self.create_vao();
            self.bind_vao(&vao);
            let shader = shader_system.get_shader(&renderable.shader_kind()).unwrap();
            renderable.buffer_attributes(gl, &shader);
            self.vao_ext.vaos.borrow_mut().insert(key.to_string(), vao);
            return;
        }

        let vaos = self.vao_ext.vaos.borrow();
        let vao = vaos.get(key).unwrap();
        self.bind_vao(vao);
    }

    fn bind_vao(&self, vao: &Vao) {
        let oes_vao_ext = &self.vao_ext.oes_vao_ext;

        let bind_vao_ext = Reflect::get(&oes_vao_ext, &"bindVertexArrayOES".into())
            .expect("Create vao func")
            .into();

        let args = js_sys::Array::new();
        args.push(&vao.0);

        Reflect::apply(&bind_vao_ext, oes_vao_ext, &args).expect("Bound VAO");
    }
}
