use std::cell::{Ref, RefCell, RefMut};
use std::convert::Into;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::usize;
use svg_load::path::RenderablePath;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

use crate::animation::Animator;
use crate::app::ui::drag::Draggable;
use crate::{EventTarget, FieldSelector};
use crate::messaging::HandleContext;
use crate::messaging::HandlerCallback;
use crate::messaging::HandlersBean;
use crate::messaging::Msg;
use crate::render::framebuffer::Framebuffer;
use crate::render::textured_quad::TexturedQuad;
use crate::render::WebRenderer;
use crate::State;
use crate::ui::element::Element;

pub mod animation;
pub mod drag;
pub mod element;
pub mod picking;
pub mod render;
pub mod path;
pub mod fields;
pub mod messaging;

pub struct UI {
    canvas: HtmlCanvasElement,
    fullscreen: bool,

    pick_fbo: Option<WebGlFramebuffer>,
    renderer: Rc<WebRenderer>,
    gl: WebGlRenderingContext,

    drag_elem: Option<usize>,
    start_drag_x: i32,
    start_drag_y: i32,

    handling: Rc<RefCell<HandlersBean>>,

    svg: Option<Vec<RenderablePath>>,
}

impl UI {
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

        let h = canvas.height();
        let w = canvas.width();
        let (_screen_texture, fbo) = Framebuffer::create_texture_frame_buffer(
            w as i32,
            h as i32,
            &gl,
        );

        let handling = HandlersBean::new(w, h);

        UI {
            canvas,
            fullscreen: false,
            pick_fbo: fbo,
            renderer,
            gl,
            drag_elem: None,
            start_drag_x: 0,
            start_drag_y: 0,
            handling: Rc::new(RefCell::new(handling)),
            svg: None,
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
            for animation in self.handle().animations.borrow_mut().deref_mut() {
                animation.animate(self.handle().elements.get(animation.get_target()).unwrap().borrow_mut().deref_mut());
            }
            self.handle().animations.borrow_mut().retain(|animation| {
                !animation.is_finished()
            });
        }

            if state.show_pick() {
                for element in self.handle().elements.iter().rev() {
                    renderer.render_picking(gl, element.borrow().deref());
                }
            } else {
                for element in self.handle().elements.iter().rev() {
                    renderer.render_mesh(gl, state, &format!("ui{}", element.borrow().get_id()), element.borrow().deref());
                }
            }
    }

    fn render_and_pick(&mut self, x: i32, y: i32) -> Option<usize> {
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

        for element in self.handle().elements.iter().rev() {
            self.renderer.render_picking(&gl, element.borrow().deref());
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
        if col>0 { Some(col) } else {None}
    }

    fn cancel_drag(&mut self) {
        console::log_1(&format!("cancel_drag").into());
        self.handle().elements[self.drag_elem.unwrap()].borrow_mut().process_cancel();
        self.drag_elem = None;
    }

    fn process_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("process_drag {} {}", x, y).into());
        self.handle().elements[self.drag_elem.unwrap()].borrow_mut().process_drag(x, y);
    }

    fn complete_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("complete_drag {} {}", x, y).into());
        self.handle().elements[self.drag_elem.unwrap()].borrow_mut().process_drop();
    }

    fn handle(&self) -> Ref<HandlersBean> {
        self.handling.as_ref().borrow()
    }

    fn handle_mut(&mut self) -> RefMut<HandlersBean> {
        self.handling.as_ref().borrow_mut()
    }
}

impl HandleContext for UI {
    fn start_animation(&self, a: Box<dyn Animator>) {
        self.handle().start_animation(a)
    }

    fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback) {
        self.handle_mut().register_handler(target_id, message_type, callback)
    }

    fn remove_handler(&mut self, target_id: usize, message_type: Msg) {
        self.handle_mut().remove_handler(target_id, message_type)
    }

    fn add_element(&mut self, elem: Element, parent_id: usize) -> Option<usize> {
        let option = self.handle_mut().add_element(elem, parent_id);
        let res = option.unwrap();
        console::log_1(&format!("Added element {} with parent {}", res, parent_id).into());
        for el in &self.handling.borrow().elements {
            let e = el.borrow();
            console::log_1(&format!("Element {} with parent {}", e.get_id(), e.parent_element).into());
        };
        Some(res)
    }

    fn remove_element(&mut self, target_id: usize) {
        self.handle_mut().remove_element(target_id)
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
                    let mut consume;

                    {
                        let handle_bean = self.handle();
                        let handler = handle_bean.get_handler(target_id, *msg);

                        consume = if handler.is_some() {
                            console::log_1(&format!("found").into());
                            handler.unwrap().borrow_mut()(msg, self.handling.clone().as_ref().borrow().deref())
                        } else {
                            console::log_1(&format!("not found {}", target_id).into());
                            false
                        }
                    }


                    if self.handle().elements[target_id].borrow().is_draggable() {
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
                if *key_code == 32 { //Spacebar
                    self.toggle_fullscreen();
                }
                false
            }
            Msg::ResizeViewport(w, h) => {
                self.canvas.set_width(*w as u32);
                self.canvas.set_height(*h as u32);
                self.handling.borrow_mut().elements.get(0).unwrap().borrow_mut().set(FieldSelector::Width(*w as u32));
                self.handling.borrow_mut().elements.get(0).unwrap().borrow_mut().set(FieldSelector::Height(*h as u32));
                self.gl.delete_framebuffer( self.pick_fbo.as_ref() );
                let (_screen_texture, fbo) = Framebuffer::create_texture_frame_buffer(*w as i32, *h as i32, &self.gl);
                self.pick_fbo = fbo;
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
