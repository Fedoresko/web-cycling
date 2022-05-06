use std::borrow::{Borrow, BorrowMut};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::convert::Into;
use std::f32::consts::PI;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::usize;

use derivative::Derivative;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

use crate::State;
use crate::animation::{Animation, Animator, CompositeAnimation};
use crate::app::ui::drag::Draggable;
use crate::app::ui::element::{ElemBuider, LineStyle, ShapeSegment};
use crate::EventTarget;
use crate::fields::{FieldSelector, Vec4};
use crate::render::framebuffer::Framebuffer;
use crate::render::textured_quad::TexturedQuad;
use crate::render::WebRenderer;
use crate::ui::element::Element;

pub mod animation;
pub mod drag;
pub mod element;
pub mod picking;
pub mod render;
pub mod path;
pub mod fields;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Derivative)]
#[derivative(Eq, PartialEq, Hash)]
pub enum Msg {
    AdvanceClock(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        f32),
    MouseDown(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    MouseUp(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    MouseMove(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    Zoom(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]f32),
    ShowScenery(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]bool),
    KeyDown(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]u32),
    KeyUp(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]u32),
    ResizeViewport(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    DragEvent(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    DropEvent(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
}

impl Display for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct UI {
    pub canvas: HtmlCanvasElement,
    fullscreen: bool,
    elements: Vec<RefCell<Element>>,
    pick_fbo: Option<WebGlFramebuffer>,
    renderer: Rc<WebRenderer>,
    gl: WebGlRenderingContext,

    drag_elem: Option<usize>,
    start_drag_x: i32,
    start_drag_y: i32,

    handling: Rc<HandlersBean>,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct EvtKey {
    target_id: usize,
    message_type: Msg,
}

pub type HandlerCallback = RefCell<Box<dyn Fn(&Msg, &HandlersBean) -> bool + 'static>>;

pub trait HandleContext {
    fn start_animation(&self, a: Box<dyn Animator>);
    fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback);
    fn remove_handler(&mut self, target_id: usize, message_type: Msg);
}

pub struct HandlersBean {
    animations: RefCell<Vec<Box<dyn Animator>>>,
    elem_handlers: HashMap<EvtKey, HandlerCallback>,
}
//<dyn FnMut(&Msg, RefMut<HandlersBean>) -> bool + 'static>
impl HandleContext for HandlersBean {

    fn start_animation(&self, animation: Box<dyn Animator>) {
        console::log_1(&format!("Starting animation").into());
        self.animations.borrow_mut().push(animation);
    }

    fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback) {
        console::log_1(&format!("Registered handler {} {}", target_id, message_type).into());
        self.elem_handlers.insert(EvtKey { target_id, message_type }, callback);
    }

    fn remove_handler(&mut self, target_id: usize, message_type: Msg) {
        self.elem_handlers.remove(&EvtKey { target_id, message_type });
    }
}

impl UI {
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

    pub fn new(canvas: HtmlCanvasElement, renderer: Rc<WebRenderer>) -> UI {
        let result = JsValue::from_serde(&serde_json::json!({
            "antialias": false,
        }));
        let gl: WebGlRenderingContext = canvas
            .get_context_with_context_options("webgl", &result.unwrap())
            .ok()
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

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

        let small_button = ElemBuider::new(600, 300, 200, 50)
            .with_shape(&shape)
            .with_background(&[0.0, 0.0, 0.0, 1.0])
            .with_line_style(&LineStyle {
                color: [1.0, 1.0, 1.0, 1.0],
                width: 1.0,
                dashed: false,
            })
            .with_gradient(3, vec![0.25, 0.5, 0.75],
                           vec![Vec4::from([0.5, 0.5, 0.5, 1.0]), Vec4::from([0.9, 0.9, 0.9, 1.0]), Vec4::from([0.5, 0.5, 0.5, 1.0])], (0.0, 0.0), (1.0, 1.0)).build();

        let star = ElemBuider::new(200, 300, 200, 200)
            .with_shape(&Self::star_shape())
            .with_line_style(&LineStyle {
                color: [0.8, 0.2, 0.0, 0.75],
                width: 2.0,
                dashed: false,
            })
            .with_background(&[0.6, 0.4, 0.0, 0.5])
            .draggable()
            .build();

        let big_box = ElemBuider::new(100, 100, canvas.width() - 200, canvas.height() - 200)
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


        let id = small_button.get_id()-1;
        let elements = vec![RefCell::new(small_button), RefCell::new(star), RefCell::new(big_box)];

        let (_screen_texture, fbo) = Framebuffer::create_texture_frame_buffer(
            canvas.width() as i32,
            canvas.height() as i32,
            &gl,
        );

        let mut handling = HandlersBean{
            animations: RefCell::new(Vec::new()),
            elem_handlers: HashMap::new(),
        };

        handling.register_handler(id, Msg::MouseDown(0, 0), RefCell::new(Box::new(move |_msg, mut context| {
            let anim1 = Box::new(Animation::linear(id,
                                                   FieldSelector::GradientPos0(-0.1), FieldSelector::GradientPos0(1.0), 1000.0));
            let anim2 = Box::new(Animation::linear(id,
                                                   FieldSelector::GradientPos1(-0.05), FieldSelector::GradientPos1(1.05), 1000.0));
            let anim3 = Box::new(Animation::linear(id,
                                                   FieldSelector::GradientPos2(0.0), FieldSelector::GradientPos2(1.1), 1000.0));
            let animations: Vec<Box<dyn Animator>> = vec![anim1, anim2, anim3];
            let button_flare = Box::new(CompositeAnimation { animations });
            context.start_animation(button_flare);
            true
        })));

        UI {
            canvas,
            fullscreen: false,
            elements,
            pick_fbo: fbo,
            renderer,
            gl,
            drag_elem: None,
            start_drag_x: 0,
            start_drag_y: 0,
            handling: Rc::new(handling),
        }
    }

    pub fn toggle_fullscreen(&mut self) {
        console::log_1(&format!("Toggling fullscreen {}", self.fullscreen).into());
        self.fullscreen = !self.fullscreen;
        if self.fullscreen {
            self.canvas
                .request_fullscreen()
                .expect("Cannot request fullscreen for canvas");
        } else {
            let window = window().unwrap();
            let document = window.document().unwrap();
            document.exit_fullscreen();
        }
    }

    pub fn render_elements(
        &self,
        gl: &WebGlRenderingContext,
        state: &State,
        renderer: &WebRenderer,
    ) {
        gl.viewport(0, 0, state.viewport_width(), state.viewport_height());
        gl.clear_color(0.0, 0.0, 0.0, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        if !state.show_pick() {
            let background = TexturedQuad::new(
                0,
                0,
                state.viewport_width() as u16,
                state.viewport_height() as u16,
                0.5,
                state.width_rate(),
                state.height_rate(),
                0,
            );

            renderer.render_mesh(gl, state, "background", &background);
        }

        {
            for animation in self.handling.animations.borrow_mut().deref_mut() {
                animation.animate(self.elements.get(animation.get_target()).unwrap().borrow_mut().deref_mut());
            }
            self.handling.animations.borrow_mut().retain(|animation| {
                !animation.is_finished()
            });
        }

        for element in &self.elements {
            if state.show_pick() {
                renderer.render_picking(gl, element.borrow().deref());
            } else {
                renderer.render_mesh(gl, state, &format!("ui{}", element.borrow().get_id()), element.borrow().deref());
            }
        }
    }

    fn render_and_pick(&mut self, x: i32, y: i32) -> Option<usize> {
        let mut elem_colors: HashMap<usize, Option<usize>> = HashMap::new();
        let gl = &self.gl;

        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.enable(GL::DEPTH_TEST);

        gl.bind_framebuffer(GL::FRAMEBUFFER, self.pick_fbo.as_ref());
        gl.viewport(
            0,
            0,
            self.canvas.width() as i32,
            self.canvas.height() as i32,
        );
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        for (pos, element) in self.elements.iter().enumerate() {
            // renderer.render_mesh(gl, state, &format!("ui{}",element.id), element);
            let col = self.renderer.render_picking(&gl, element.borrow().deref());
            elem_colors.insert(col, Some(pos));
        }

        let pixel_x = x * self.canvas.width() as i32 / self.canvas.client_width() as i32;
        let pixel_y = y * self.canvas.height() as i32 / self.canvas.client_height() as i32;
        let mut data: [u8; 4] = [0, 0, 0, 0];
        gl.read_pixels_with_opt_u8_array(
            pixel_x,           // x
            pixel_y,           // y
            1,                 // width
            1,                 // height
            GL::RGBA,          // format
            GL::UNSIGNED_BYTE, // type
            Some(&mut data),
        )
            .expect("Cannot pick color from GL context");

        let col = usize::from_be_bytes(data) >> 8;

        console::log_1(
            &format!(
                "Got from pick: {} {} {} {} mouse: {} {} px: {} {}",
                data[0], data[1], data[2], data[3], x, y, pixel_x, pixel_y
            )
                .into(),
        );

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        elem_colors.remove(&col).unwrap_or(None)
    }

    fn cancel_drag(&mut self) {
        console::log_1(&format!("cancel_drag").into());
        self.elements[self.drag_elem.unwrap()].borrow_mut().process_cancel();
        self.drag_elem = None;
    }

    fn process_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("process_drag {} {}", x, y).into());
        self.elements[self.drag_elem.unwrap()].borrow_mut().process_drag(x, y);
    }

    fn complete_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("complete_drag {} {}", x, y).into());
        self.elements[self.drag_elem.unwrap()].borrow_mut().process_drop();
    }
}


impl EventTarget for UI {
    fn msg(&mut self, msg: &Msg) -> bool {
        match msg {
            Msg::MouseDown(x, y) => {
                console::log_1(&format!("mouse_down").into());

                let x = *x;
                let y = self.canvas.height() as i32 - y;

                let pick = self.render_and_pick(x, y);

                if pick.is_some() {
                    let target_id = pick.unwrap();
                    let mut consume = false;

                    {
                        let handler= self.handling.elem_handlers.get(&EvtKey { target_id, message_type: *msg });

                        consume = if (handler.is_some()) {
                            console::log_1(&format!("found").into());
                            handler.unwrap().borrow_mut()(msg, self.handling.clone().deref())
                        } else {
                            console::log_1(&format!("not found {}", target_id).into());
                            false
                        }
                    }


                    if self.elements[target_id].borrow().is_draggable() {
                        console::log_1(&format!("starting drag for {}", target_id).into());
                        self.drag_elem = pick;
                        self.start_drag_x = x;
                        self.start_drag_y = y;
                        self.canvas
                            .style()
                            .set_property("cursor", "move")
                            .expect("Cannot set cursor pointer");
                        consume |= true;
                    } else {
                        if self.drag_elem.is_some() {
                            self.cancel_drag();
                        }
                    }
                    consume
                } else { false }
            }
            Msg::KeyDown(key_code) => {
                if *key_code == 32 {
                    self.toggle_fullscreen();
                }
                false
            }
            Msg::ResizeViewport(w, h) => {
                self.canvas.set_width(*w as u32);
                self.canvas.set_height(*h as u32);
                false
            }
            Msg::MouseMove(x, y) => {
                let x = *x - self.start_drag_x;
                let y = (self.canvas.height() as i32 - y) - self.start_drag_y;
                if self.drag_elem.is_some() {
                    self.process_drag(x, y);
                    true
                } else {
                    false
                }
            }
            Msg::MouseUp(x, y) => {
                let x = *x;
                let y = self.canvas.height() as i32 - y;
                if self.drag_elem.is_some() {
                    self.complete_drag(x, y);
                    self.drag_elem = None;
                    self.canvas
                        .style()
                        .set_property("cursor", "default")
                        .expect("Cannot set cursor pointer");
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
