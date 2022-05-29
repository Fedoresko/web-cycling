extern crate wasm_bindgen;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::rc::Rc;

use console_error_panic_hook;
use js_sys::Date;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use app::ui::messaging::EventTarget;
use crate::animation::{Animation, Animator, CompositeAnimation};
use crate::bluetooth::hrm::HRM;
use crate::element::{ElemBuilder, LineStyle, ShapeSegment};
use crate::fields::{FieldSelector, SizedStr, Vec4};

use crate::load_texture_img::load_texture_image;
use crate::render::framebuffer::Framebuffer;

pub(in crate) use self::app::*;
use self::app::ui::UI;
use self::canvas::*;
use self::render::*;
use crate::messaging::{HandlerImpact, Msg};
use crate::fields::Sizing;

mod app;
mod canvas;
mod load_texture_img;
mod render;
mod shader;

trait WC {
    fn update(&mut self, dt: f32);
    fn render(&self);
    fn load_textures(&self);
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

struct InnerWebClient {
    app: Rc<App>,
    gl: Rc<WebGl2RenderingContext>,
    renderer: Rc<WebRenderer>,
    event_dispatcher: Rc<RefCell<Option<WebEventDispatcher>>>,
    screen_texture: Option<WebGlTexture>,
    fbo: Option<WebGlFramebuffer>,
    renderbuffer: Option<WebGlFramebuffer>,
    colorbuffer: Option<WebGlFramebuffer>,

    fps_label_id: usize,
    //hr_label_id: usize,
    last_render_times: VecDeque<f32>,
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

        let fps_label_id = Self::create_fps_label(w, h, &mut ui);
        //let heart_rate_id = Self::create_hr_control(h, &mut ui);

        let dispatcher = WebEventDispatcher {
            app: app.clone(),
            ui,
        };

        *event_dispatcher.borrow_mut() = Some(dispatcher);

        // let (screen_texture, fbo) =
        //     Framebuffer::create_texture_frame_buffer(scr_width, scr_height, &gl);

        let (screen_texture, renderbuffer, colorbuffer) = Framebuffer::create_framebuffers_multisampling(scr_width, scr_height, &gl);
        let fbo = Framebuffer::create_msaa_fbo(scr_width, scr_height, &gl);

        let gl_ref = Rc::new(gl);
        InnerWebClient {
            app,
            gl: gl_ref,
            renderer,
            event_dispatcher: event_dispatcher.clone(),
            screen_texture,
            fbo,
            renderbuffer,
            colorbuffer,
            fps_label_id,
            //hr_label_id: heart_rate_id,
            last_render_times: VecDeque::new()
        }
    }


    fn create_fps_label(w: u32, h: u32, ui: &mut UI) -> usize {
        let fps_label = ElemBuilder::new(w as i32 - 100, h as i32 - 20, 100, 20).with_background(&[0.0, 0.0, 0.0, 1.0])
            .with_label("0 FPS", "Roboto-Light", 16.0, Vec4::from([1.0, 1.0, 1.0, 1.0])).build();
        let fps_label_id = ui.add_element(fps_label, 0).unwrap();
        ui.add_bind(0, fps_label_id, Box::new(|fs: &FieldSelector| {
            if let FieldSelector::Height(h) = *fs {
                return Some(vec![FieldSelector::Y(h as i32 - 20)]);
            } else if let FieldSelector::Width(w) = *fs {
                return Some(vec![FieldSelector::X(w as i32 - 100)]);
            }
            None
        }));
        fps_label_id
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
            .with_background(&[0.0, 0.0, 0.0, 1.0])
            .with_gradient(2, vec![0.0, 1.0], vec![Vec4::from([0.0, 0.0, 0.0, 1.0]), Vec4::from([0.0, 0.0, 0.0, 0.5])], (0.3, 0.1), (0.7, 0.9))
            .build();

        //let _big_box_id = ui.add_element(big_box, 0).unwrap();
        // ui.add_bind(0, _big_box_id, Box::new(|fs : &FieldSelector| {
        //     if let FieldSelector::Height(h) = *fs {
        //         return Some(FieldSelector::Height(h - 200));
        //     } else if let FieldSelector::Width(w) = *fs {
        //         return Some(FieldSelector::Width(w - 200));
        //     }
        //    None
        // }));


        let _star_id = ui.add_element(star, 0).unwrap();
        let small_button_id = ui.add_element(small_button, 0).unwrap();
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

        let ac_logo = ElemBuilder::new(400, 200, 400, 250)
            .svg("test.svg").build();
        let svg_id = ui.add_element(ac_logo, 0).unwrap();

        let anim1 = Box::new(Animation::linear(svg_id, FieldSelector::Y(h as i32), FieldSelector::Y(-250), 2000.0));
        let anim2 = Box::new(Animation::fade_in_out(svg_id, FieldSelector::BGColor(Vec4::from([0.0,0.0,0.0,0.0])),
                                                    FieldSelector::BGColor(Vec4::from([0.0,0.0,0.0,1.0])), 2000.0));
        let anim3 = Box::new(Animation::fade_in_out(svg_id, FieldSelector::Width(400), FieldSelector::Width(600), 2000.0));
        let anim4 = Box::new(Animation::fade_in_out(svg_id, FieldSelector::Height(250), FieldSelector::Height(375), 2000.0));
        let anim5 = Box::new(Animation::fade_in_out(svg_id, FieldSelector::X(w as i32 / 2 - 200), FieldSelector::X(w as i32 / 2 - 300), 2000.0));
        ui.start_animation( Box::new(CompositeAnimation { animations: vec![anim1, anim2, anim3, anim4, anim5] }), Some(
            Box::new(move |_m| {
                HandlerImpact::RemoveElement(svg_id)
            })
        ));



    }
}

impl WC for InnerWebClient {
    /// Update our simulation

    fn update(&mut self, dt: f32) {
        let evt = self.event_dispatcher.borrow();
        let ui = &evt.as_ref().unwrap().ui;

        self.last_render_times.push_back(dt);
        if self.last_render_times.len() > 30 {
            self.last_render_times.pop_front();
        }
        let avg = self.last_render_times.iter().sum::<f32>() / self.last_render_times.len() as f32;
        ui.set(self.fps_label_id, &FieldSelector::LabelText(SizedStr::sizify(&format!("FPS {}", (1000.0 / avg) as u32 )) ) );

        self.app.store.as_ref().borrow_mut().msg(&Msg::AdvanceClock(dt));
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    fn render(&self) {
        let x = self.event_dispatcher.borrow();
        let ui = &x.as_ref().unwrap().ui;
        let gl = &self.gl;

        //Draw 3D scene
        // gl.bind_framebuffer(GL::FRAMEBUFFER, self.fbo.as_ref());
        // self.renderer
        //     .render(gl, &self.app.store.borrow().state);

        gl.bind_framebuffer(GL::FRAMEBUFFER, self.renderbuffer.as_ref());

        self.renderer
            .render(gl, &self.app.store.borrow().state);

        let w = gl.drawing_buffer_width();
        let h = gl.drawing_buffer_height();

        gl.bind_framebuffer(GL::READ_FRAMEBUFFER, self.renderbuffer.as_ref());
        gl.bind_framebuffer(GL::DRAW_FRAMEBUFFER, self.colorbuffer.as_ref());
        gl.clear_bufferfv_with_f32_array(GL::COLOR, 0, &[0.0, 0.0, 0.0, 1.0]);
        gl.blit_framebuffer(
            0, 0, w, h,
            0, 0, w, h,
            GL::COLOR_BUFFER_BIT, GL::NEAREST
        );

        gl.bind_framebuffer(GL::READ_FRAMEBUFFER, self.fbo.as_ref());
        gl.bind_framebuffer(GL::DRAW_FRAMEBUFFER, self.colorbuffer.as_ref());
        gl.clear_bufferfv_with_f32_array(GL::DEPTH, 0, &[0.0, 0.0, 0.0, 1.0]);
        gl.blit_framebuffer(
            0, 0, w, h,
            0, 0, w, h,
            GL::DEPTH_BUFFER_BIT, GL::NEAREST
        );

        //Apply filters and draw UI
        gl.bind_framebuffer(GL::FRAMEBUFFER, self.fbo.as_ref());
        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, self.screen_texture.as_ref());

        ui.render_elements(gl, &self.app.store.borrow().state, &self.renderer);

        gl.bind_framebuffer(GL::READ_FRAMEBUFFER, self.fbo.as_ref());
        gl.bind_framebuffer(GL::DRAW_FRAMEBUFFER, None);
        gl.clear_bufferfv_with_f32_array(GL::COLOR, 0, &[0.0, 0.0, 0.0, 1.0]);
        gl.blit_framebuffer(
            0, 0, w, h,
            0, 0, w, h,
            GL::COLOR_BUFFER_BIT, GL::NEAREST
        );
    }

    fn load_textures(&self) {
        let gl = &self.gl;

        load_texture_image(
            Rc::clone(gl),
            "/track_texture2.png",
            TextureUnit::VelodromeFlat,
            true,
        );
        load_texture_image(
            Rc::clone(gl),
            "/normalmap.png",
            TextureUnit::NormalMap,
            false,
        );
        load_texture_image(Rc::clone(gl), "/sky.jpg", TextureUnit::Stad, true);
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

            wcc.borrow_mut().update(dt as f32);
            wcc.as_ref().borrow_mut().render();

            time = Date::now();

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
}
