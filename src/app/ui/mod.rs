use std::cell::{Ref, RefCell, RefMut};
use std::convert::Into;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::usize;
use svg_load::path::RenderablePath;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use messaging::EventTarget;

use crate::animation::Animator;
use crate::app::ui::drag::Draggable;
use crate::{FieldSelector, HRM};
use crate::components::{Component, UserEvent};
use crate::components::UserEvent::{HrChanged, ProcessDrag, ProcessDrop};
use crate::messaging::HandlerCallback;
use crate::messaging::HandlersBean;
use crate::messaging::Msg;
use crate::render::framebuffer::Framebuffer;
use crate::render::textured_quad::TexturedQuad;
use crate::render::WebRenderer;
use crate::State;
use crate::ui::element::{Element, UINode};
use std::convert::TryFrom;

pub mod animation;
pub mod drag;
pub mod element;
pub mod picking;
pub mod render;
pub mod path;
pub mod fields;
pub mod messaging;
pub mod text;
pub mod geom;
pub mod chart;

pub struct UI {
    canvas: HtmlCanvasElement,
    fullscreen: bool,

    pick_fbo: Option<WebGlFramebuffer>,
    renderer: Rc<WebRenderer>,
    gl: WebGl2RenderingContext,

    drag_elem: Option<usize>,
    start_drag_x: i32,
    start_drag_y: i32,

    handling: Rc<RefCell<HandlersBean>>,

    _svg: Option<Vec<RenderablePath>>,
}

impl UI {
    pub fn new(canvas: HtmlCanvasElement, renderer: Rc<WebRenderer>) -> UI {
        let result = JsValue::from_serde(&serde_json::json!({
            "antialias": false,
        }));
        let gl: WebGl2RenderingContext = canvas
            .get_context_with_context_options("webgl2", &result.unwrap())
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

        let handling = HandlersBean::new(w as i32, h as i32);

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
            _svg: None,
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
        gl: &WebGl2RenderingContext,
        state: &State,
        renderer: &WebRenderer,
    ) {
        gl.viewport(0, 0, state.viewport_width(), state.viewport_height());
        gl.clear_color(0.0, 0.0, 0.0, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        gl.enable(GL::BLEND);

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
                for res in animation.animator.animate() {
                    self.handle().set(animation.animator.get_target(), res);
                }
            }

            self.handle_mut().remove_finished_animations();
        }


        // for (k, mesh) in assets.get_image("HR").expect("SVG").iter().rev().enumerate() {
        //     renderer.render_mesh(gl, state, &format!("test1{}",k), mesh);
        // }


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

        gl.bind_framebuffer(GL::FRAMEBUFFER, self.pick_fbo.as_ref());
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.enable(GL::DEPTH_TEST);
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

        let col = u32::from_be_bytes(data) >> 8;

        console::log_1(
            &format!(
                "Got from pick: {} {} {} {} mouse: {} {} px: {} {}",
                data[0], data[1], data[2], data[3], x, y, pixel_x, pixel_y
            )
                .into(),
        );

        gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        if col>0 { Some(usize::try_from(col).unwrap()) } else {None}
    }

    fn cancel_drag(&mut self) {
        console::log_1(&format!("cancel_drag").into());
        self.handle().elem_by_id(self.drag_elem.unwrap()).borrow_mut().process_cancel();
        self.drag_elem = None;
    }

    fn process_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("process_drag {} {}", x, y).into());
        self.handle().elem_by_id(self.drag_elem.unwrap()).borrow_mut().process_drag(x, y);
        self.handle_mut().push_event(ProcessDrag((self.drag_elem.unwrap(),x,y)));
    }

    fn complete_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("complete_drag {} {}", x, y).into());
        self.handle().elem_by_id(self.drag_elem.unwrap()).borrow_mut().process_drop();
        self.handle_mut().push_event(ProcessDrop((self.drag_elem.unwrap(),x,y)));
    }

    fn handle(&self) -> Ref<HandlersBean> {
        self.handling.borrow()
    }

    fn handle_mut(&self) -> RefMut<HandlersBean> {
        self.handling.borrow_mut()
    }

    pub fn add_bind(&mut self, source_id: usize, target_id: usize, map_fn: Box<dyn Fn(&FieldSelector) -> Option<Vec<FieldSelector>>>) {
        self.handle_mut().add_bind(source_id, target_id, map_fn);
    }

    pub fn start_animation(&self, a: Box<dyn Animator>, on_finish: Option<HandlerCallback>) {
        self.handle().start_animation(a, on_finish)
    }

    pub fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback) {
        self.handle_mut().register_handler(target_id, message_type, callback)
    }

    pub fn remove_handler(&mut self, target_id: usize, message_type: Msg) {
        self.handle_mut().remove_handler(target_id, message_type)
    }

    pub fn add_element(&mut self, elem: Element, parent_id: usize) -> Option<usize> {
        let option = self.handle_mut().add_element(elem, parent_id);
        let res = option.unwrap();
        console::log_1(&format!("Added element {} with parent {}", res, parent_id).into());
        for el in &self.handle().elements {
            let e = el.borrow();
            console::log_1(&format!("Element {} with parent {}", e.get_id(), e.parent_element).into());
        };
        Some(res)
    }

    pub fn remove_element(&mut self, target_id: usize) {
        self.handle_mut().remove_element(target_id)
    }

    pub fn set(&self, target_id: usize, value: FieldSelector) {
        self.handle().set(target_id, value);
    }

    pub fn add_component<T>(&mut self, component: T, parent: usize) -> usize where T : Component + 'static {
        self.handling.borrow_mut().add_component(component, parent)
    }

    pub fn emit(&self, event: UserEvent) {
        self.handling.borrow_mut().push_event(event);
    }
}

impl EventTarget for UI {
    fn msg(&mut self, msg: &Msg) -> bool {
        match msg {
            Msg::AdvanceClock(dt) => {
                self.handling.as_ref().borrow_mut().process_events();
                false
            }
            Msg::MouseDown(x, y) => {
                console::log_1(&format!("mouse_down").into());

                let x = *x;
                let y = self.canvas.height() as i32 - y;

                let pick = self.render_and_pick(x, y);

                if pick.is_some() {
                    let target_id = pick.unwrap();
                    let mut consume;

                    let handler_impact;
                    {
                        let handle_bean = self.handle();
                        handler_impact = handle_bean.get_handler(target_id, *msg).map( | h| h(msg) );
                    }
                    consume = if let Some(impact) = handler_impact {
                        console::log_1(&format!("found").into());
                        self.handle_mut().process_impact( impact );
                        true
                    } else {
                        console::log_1(&format!("not found {}", target_id).into());
                        false
                    };

                    if self.handle().elem_by_id(target_id).borrow().is_draggable() {
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
                } else if *key_code == 67 { //'C'
                    let mut handle = self.handling.clone();
                    let hrm = HRM::new(move|js| {
                        let hr = js.as_f64().unwrap();
                        handle.borrow_mut().push_event(HrChanged(hr as i32))
                    }, |js| {});
                    hrm.reconnect_hrm();
                }
                false
            }
            Msg::ResizeViewport(w, h) => {
                self.canvas.set_width(*w as u32);
                self.canvas.set_height(*h as u32);
                self.handle().set(0, FieldSelector::Width(*w));
                self.handle().set(0, FieldSelector::Height(*h));
                self.gl.delete_framebuffer( self.pick_fbo.as_ref() );
                let (_screen_texture, fbo) = Framebuffer::create_texture_frame_buffer(*w, *h, &self.gl);
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
