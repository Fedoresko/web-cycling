extern crate wasm_bindgen;

use std::cell::RefCell;
use std::rc::Rc;

use console_error_panic_hook;
use js_sys::Date;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

use ui::EventTarget;

use crate::load_texture_img::load_texture_image;
use crate::render::framebuffer::Framebuffer;

pub(in crate) use self::app::*;
use self::app::ui::UI;
use self::canvas::*;
use self::render::*;

mod app;
mod canvas;
mod load_texture_img;
mod render;
mod shader;

trait WC {
    fn update(&self, dt: f32);
    fn render(&self);
    fn load_textures(&self);
}

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
    wc: Rc<dyn WC>,
}

///Dispatch UI events
pub struct WebEventDispatcher {
    app: Rc<App>,
    ui: UI,
}

impl EventTarget for WebEventDispatcher {
    fn msg(&mut self, msg: &Msg) -> bool {
        if !self.ui.msg(msg) {
            self.app.store.borrow_mut().msg(msg);
        }
        true
    }
}

struct InnerWebClient {
    app: Rc<App>,
    gl: Rc<WebGlRenderingContext>,
    renderer: Rc<WebRenderer>,
    event_dispatcher: Rc<RefCell<Option<WebEventDispatcher>>>,
    screen_texture: Option<WebGlTexture>,
    fbo: Option<WebGlFramebuffer>,
}

impl InnerWebClient {
    fn new() -> InnerWebClient {
        console_error_panic_hook::set_once();
        let scr = window().screen().unwrap();
        let scr_width = scr.width().unwrap();
        let scr_height = scr.height().unwrap();

        let event_dispatcher = Rc::new(RefCell::new(None));
        let canvas = init_canvas(&event_dispatcher).unwrap();
        let app = Rc::new(App::new(
            canvas.width() as i32,
            canvas.height() as i32,
            scr_width,
            scr_height,
        ));
        //let ui_ref = Rc::new(&dispatcher.ui);
        let gl = create_webgl_context(&canvas).unwrap();
        let renderer = Rc::new(WebRenderer::new(&gl));
        let dispatcher = WebEventDispatcher {
            app: app.clone(),
            ui: UI::new(canvas, Rc::clone(&renderer)),
        };

        *event_dispatcher.borrow_mut() = Some(dispatcher);

        let (screen_texture, fbo) =
            Framebuffer::create_texture_frame_buffer(scr_width, scr_height, &gl);

        let gl_ref = Rc::new(gl);
        InnerWebClient {
            app,
            gl: gl_ref,
            renderer,
            event_dispatcher: event_dispatcher.clone(),
            screen_texture,
            fbo,
        }
    }
}

impl WC for InnerWebClient {
    /// Update our simulation
    fn update(&self, dt: f32) {
        self.app.store.borrow_mut().msg(&Msg::AdvanceClock(dt));
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    fn render(&self) {
        let x = self.event_dispatcher.borrow();
        let ui = &x.as_ref().unwrap().ui;
        let gl = &self.gl;

        //let w = self.app.store.borrow().state.viewport_width();
        //let h = self.app.store.borrow().state.viewport_height();
        //let (target_texture, fb) = InnerWebClient::create_texture_frame_buffer(w, h, gl);

        //Draw 3D scene
        gl.bind_framebuffer(GL::FRAMEBUFFER, self.fbo.as_ref());

        self.renderer
            .render(gl, &self.app.store.borrow().state, &self.app.assets());

        //Apply filters and draw UI
        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, self.screen_texture.as_ref());

        ui.render_elements(gl, &self.app.store.borrow().state, &self.renderer);
    }

    fn load_textures(&self) {
        let gl = &self.gl;

        load_texture_image(
            Rc::clone(gl),
            "/stone-texture.png",
            TextureUnit::Stone,
            false,
        );
        load_texture_image(
            Rc::clone(gl),
            "/normalmap.png",
            TextureUnit::NormalMap,
            false,
        );
        load_texture_image(Rc::clone(gl), "/stad.png", TextureUnit::Stad, false);
        load_texture_image(
            Rc::clone(gl),
            "/track_texture1024.png",
            TextureUnit::Velodrome,
            true,
        );
    }
}

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

/// Main Class - entry point for web application
#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        WebClient {
            wc: Rc::new(InnerWebClient::new()),
        }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        self.wc.load_textures();
        let wcc = self.wc.clone();

        let mut time = Date::now();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let dt = Date::now() - time;

            wcc.update(dt as f32);
            wcc.render();

            time = Date::now();

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
}
