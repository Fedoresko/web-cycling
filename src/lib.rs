extern crate wasm_bindgen;

use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

use console_error_panic_hook;
use js_sys::Date;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use crate::animation::{Animation, Animator, CompositeAnimation};
use crate::element::{ElemBuilder, LineStyle, ShapeSegment};
use crate::fields::{FieldSelector, Vec4};

use crate::load_texture_img::load_texture_image;
use crate::render::framebuffer::Framebuffer;

pub(in crate) use self::app::*;
use self::app::ui::UI;
use self::canvas::*;
use self::render::*;
use crate::messaging::{HandlerImpact, Msg};

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

pub trait EventTarget {
    fn msg(&mut self, msg: &Msg) -> bool;
}

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
    wc: Rc<RefCell<dyn WC>>,
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
        let w = canvas.width();
        let h = canvas.height();
        let app = Rc::new(App::new(
            w as i32,
            h as i32,
            scr_width,
            scr_height,
        ));
        //let ui_ref = Rc::new(&dispatcher.ui);
        let gl = create_webgl_context(&canvas).unwrap();
        let renderer = Rc::new(WebRenderer::new(&gl));
        let mut ui = UI::new(canvas, Rc::clone(&renderer));

        Self::init_ui(&mut ui, w, h);

        let dispatcher = WebEventDispatcher {
            app: app.clone(),
            ui,
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

    fn star_shape() -> Vec<ShapeSegment> {
        let mut shape: Vec<ShapeSegment> = vec![];
        for k in 0..5 {
            shape.push(ShapeSegment {
                x: (0.5 * (2.0 * PI * k as f32 / 5.0 + PI / 2.0).cos()) + 0.5,
                y: (0.5 * (2.0 * PI * k as f32 / 5.0 + PI / 2.0).sin()) + 0.5,
                style: None,
                event_id: None,
            });
            shape.push(ShapeSegment {
                x: (0.25 * (2.0 * PI * (k as f32 + 0.5) / 5.0 + PI / 2.0).cos()) + 0.5,
                y: (0.25 * (2.0 * PI * (k as f32 + 0.5) as f32 / 5.0 + PI / 2.0).sin()) + 0.5,
                style: None,
                event_id: None,
            });
        }
        shape.push(ShapeSegment {
            x: (0.5 * (PI / 2.0).cos()) + 0.5,
            y: (0.5 * (PI / 2.0).sin()) + 0.5,
            style: None,
            event_id: None,
        });
        shape
    }

    fn init_ui(ui: &mut UI, w: u32, h: u32) {
        let coords: Vec<(f32, f32)> =
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)];
        let shape: Vec<ShapeSegment> = coords
            .iter()
            .map(|(x, y)| ShapeSegment {
                x: *x,
                y: *y,
                style: None,
                event_id: None,
            })
            .collect();
        let small_button = ElemBuilder::new(600, 300, 200, 50)
            .with_shape(&shape)
            .with_background(&[0.0, 0.0, 0.0, 1.0])
            .with_line_style(&LineStyle {
                color: [1.0, 1.0, 1.0, 1.0],
                width: 1.0,
                dashed: false,
            })
            .with_gradient(3, vec![0.0, 0.0, 0.0],
                           vec![Vec4::from([0.5, 0.5, 0.5, 1.0]), Vec4::from([0.9, 0.9, 0.9, 1.0]), Vec4::from([0.5, 0.5, 0.5, 1.0])], (0.0, 0.4), (1.0, 0.6)).build();

        let star = ElemBuilder::new(200, 300, 200, 200)
            .with_shape(&Self::star_shape())
            .with_line_style(&LineStyle {
                color: [0.8, 0.2, 0.0, 0.75],
                width: 2.0,
                dashed: false,
            })
            .with_background(&[0.6, 0.4, 0.0, 0.5])
            .draggable()
            .build();

        let big_box = ElemBuilder::new(100, 100, w - 200, h - 200)
            .with_shape(&shape)
            .with_line_style(&LineStyle {
                color: [1.0, 1.0, 1.0, 1.0],
                width: 1.0,
                dashed: false,
            })
            .blur_on()
            .with_background(&[0.0, 0.0, 0.0, 0.5])
            .with_gradient(2, vec![0.0, 1.0], vec![Vec4::from([0.0, 0.0, 0.0, 0.9]), Vec4::from([0.0, 0.0, 0.0, 0.1])], (0.3, 0.1), (0.7, 0.9))
            .build();

        let _big_box_id = ui.add_element(big_box, 0).unwrap();
        let _star_id = ui.add_element(star, 0).unwrap();
        let small_button_id = ui.add_element(small_button, 0).unwrap();

        let ac_logo = ElemBuilder::new(400, 200, 300, 250)
            .with_background(&[0.0,0.0,0.0,1.0]).svg("test.svg").build();
        let svg_id = ui.add_element(ac_logo, 0).unwrap();

        let anim1 = Box::new(Animation::linear(svg_id, FieldSelector::X(0), FieldSelector::X(w as i32 - 300), 1000.0));
        let anim2 = Box::new(Animation::fade_in_out(svg_id, FieldSelector::BGColor(Vec4::from([0.0,0.0,0.0,0.0])),
                                                    FieldSelector::BGColor(Vec4::from([0.0,0.0,0.0,1.0])), 1000.0));
        // let callback : HandlerCallbackMut = RefCell::new(Box::new(move |_msg, context| {
        //     context.remove_element(svg_id);
        //     true
        // } ));
        ui.start_animation( Box::new(CompositeAnimation { animations: vec![anim1, anim2] }), Some(
            Box::new(move |_m| {
                HandlerImpact::RemoveElement(svg_id)
            })
        ));

        ui.register_handler(small_button_id, Msg::MouseDown(0, 0), Box::new(move |_msg| {
            let anim1 = Box::new(Animation::linear(small_button_id,
                                                   FieldSelector::GradientPos0(-0.6), FieldSelector::GradientPos0(1.0), 400.0));
            let anim2 = Box::new(Animation::linear(small_button_id,
                                                   FieldSelector::GradientPos1(-0.3), FieldSelector::GradientPos1(1.3), 400.0));
            let anim3 = Box::new(Animation::linear(small_button_id,
                                                   FieldSelector::GradientPos2(0.0), FieldSelector::GradientPos2(1.6), 400.0));
            let animations: Vec<Box<dyn Animator>> = vec![anim1, anim2, anim3];
            let button_flare = Box::new(CompositeAnimation { animations });
            HandlerImpact::StartAnimation(button_flare, None)
        }));

        ui.add_bind(0, _big_box_id, Box::new(|fs : &FieldSelector| {
            if let FieldSelector::Height(h) = *fs {
                return Some(FieldSelector::Height(h - 200));
            } else if let FieldSelector::Width(w) = *fs {
                return Some(FieldSelector::Width(w - 200));
            }
           None
        }));
    }
}

impl WC for InnerWebClient {
    /// Update our simulation
    fn update(&self, dt: f32) {
        self.app.store.as_ref().borrow_mut().msg(&Msg::AdvanceClock(dt));
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    fn render(&self) {
        let x = self.event_dispatcher.borrow();
        let ui = &x.as_ref().unwrap().ui;
        let gl = &self.gl;

        //Draw 3D scene
        gl.bind_framebuffer(GL::FRAMEBUFFER, self.fbo.as_ref());

        self.renderer
            .render(gl, &self.app.store.borrow().state);

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
            wc: Rc::new(RefCell::new(InnerWebClient::new())),
        }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        self.wc.borrow().load_textures();
        let wcc = self.wc.clone();

        let mut time = Date::now();
        *g.as_ref().borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let dt = Date::now() - time;

            wcc.borrow().update(dt as f32);
            wcc.as_ref().borrow_mut().render();

            time = Date::now();

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
}
