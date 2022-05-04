use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::Into;
use std::f32::consts::PI;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

use crate::{State, WebEventDispatcher};
use crate::app::ui::drag::Draggable;
use crate::app::ui::element::{ElemBuider, LineStyle, ShapeSegment};
use crate::app::ui::render::RenderableElement;
use crate::render::WebRenderer;
use crate::render::framebuffer::Framebuffer;
use crate::render::textured_quad::TexturedQuad;
use crate::ui::element::Element;

pub mod animation;
pub mod drag;
pub mod element;
pub mod picking;
pub mod render;
pub mod path;

pub trait EventTarget {
    fn msg(&mut self, msg: &Msg) -> bool;
}

pub(crate) type EventDispatcher<'a> = Rc<RefCell<Option<WebEventDispatcher>>>;

pub fn send_msg(dispatcher: &EventDispatcher, msg: &Msg) {
    Rc::clone(dispatcher)
        .borrow_mut()
        .as_mut()
        .unwrap()
        .msg(msg);
}

#[allow(dead_code)]
pub enum Msg {
    AdvanceClock(f32),
    MouseDown(i32, i32),
    MouseUp(i32, i32),
    MouseMove(i32, i32),
    Zoom(f32),
    ShowScenery(bool),
    KeyDown(u32),
    KeyUp(u32),
    ResizeViewport(i32, i32),
    DragEvent(i32, i32),
    DropEvent(i32, i32),
}

pub struct UI {
    pub canvas: HtmlCanvasElement,
    fullscreen: bool,
    elements: Vec<Element>,
    pick_fbo: Option<WebGlFramebuffer>,
    renderer: Rc<WebRenderer>,
    gl: WebGlRenderingContext,

    drag_elem: Option<usize>,
    start_drag_x: i32,
    start_drag_y: i32,
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

        let big_box = ElemBuider::new(100, 100, canvas.width() - 200, canvas.height() - 200)
            .with_shape(&shape)
            .with_line_style(&LineStyle {
                color: [1.0, 1.0, 1.0, 1.0],
                width: 1.0,
                dashed: false,
            })
            .blur_on()
            .with_background(&[0.0, 0.0, 0.0, 0.5])
            .with_gradient(2, vec![0.0,1.0], vec![[0.0, 0.0, 0.0, 0.9], [0.0, 0.0, 0.0, 0.1]], (0.3,0.1), (0.7,0.9) )
            .build();

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

        let elements = vec![star, big_box];

        let (_screen_texture, fbo) = Framebuffer::create_texture_frame_buffer(
            canvas.width() as i32,
            canvas.height() as i32,
            &gl,
        );

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

        for element in &self.elements {
            if state.show_pick() {
                renderer.render_picking(gl, element);
            } else {
                renderer.render_mesh(gl, state, &format!("ui{}", element.get_id()), element);
            }
        }
    }

    fn render_and_pick(&mut self, x: i32, y: i32) -> Option<usize> {
        let mut elem_colors: HashMap<u32, Option<usize>> = HashMap::new();
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
            let col = self.renderer.render_picking(&gl, element);
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

        let col = u32::from_be_bytes(data) >> 8;

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
        self.elements[self.drag_elem.unwrap()].process_cancel();
        self.drag_elem = None;
    }

    fn process_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("process_drag {} {}", x, y).into());
        self.elements[self.drag_elem.unwrap()].process_drag(x, y);
    }

    fn complete_drag(&mut self, x: i32, y: i32) {
        console::log_1(&format!("complete_drag {} {}", x, y).into());
        self.elements[self.drag_elem.unwrap()].process_drop();
    }
}

impl EventTarget for UI {
    fn msg(&mut self, msg: &Msg) -> bool {
        match msg {
            Msg::MouseDown(x, y) => {
                console::log_1(&format!("mouse_down").into());

                let x = *x;
                let y = self.canvas.height() as i32 - y;

                let elem = self.render_and_pick(x, y);

                if elem.is_some() && self.elements[elem.unwrap()].is_draggable() {
                    console::log_1(&format!("starting drag for {}", elem.unwrap()).into());
                    self.drag_elem = elem;
                    self.start_drag_x = x;
                    self.start_drag_y = y;
                    self.canvas
                        .style()
                        .set_property("cursor", "move")
                        .expect("Cannot set cursor pointer");
                    true
                } else {
                    if self.drag_elem.is_some() {
                        self.cancel_drag();
                    }
                    false
                }
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
