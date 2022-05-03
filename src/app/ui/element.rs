use wasm_bindgen::__rt::core::sync::atomic::{AtomicU32, Ordering};

use crate::app::ui::drag::DraggableElement;
use crate::app::ui::render::RenderableElement;

#[derive(Clone, Copy)]
pub struct LineStyle {
    pub color: [f32; 4],
    pub width: f32,
    pub dashed: bool,
}

#[derive(Clone, Copy)]
pub struct ShapeSegment {
    pub x: f32,
    pub y: f32,
    pub style: Option<LineStyle>,
    pub event_id: Option<i32>,
}

pub struct Element {
    id: u32,
    shape: Vec<ShapeSegment>,
    style: LineStyle,
    blur: bool,
    bgcolor: [f32; 4],
    pub x: i32,
    pub y: i32,
    width: u32,
    height: u32,
    children_elems: Vec<Box<Element>>,
    pub draggable: Option<DraggableElement>,
}

pub struct ElemBuider {
    id: u32,
    shape: Vec<ShapeSegment>,
    style: LineStyle,
    blur: bool,
    bgcolor: [f32; 4],
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    draggable: bool,
}

impl Element {
    pub fn children(&self) -> &[Box<Element>] {
        self.children_elems.as_slice()
    }

    pub fn add_child(&mut self, child: Element) {
        self.children_elems.push(Box::new(child))
    }
}

impl Default for LineStyle {
    fn default() -> Self {
        LineStyle {
            color: [1.0, 1.0, 1.0, 1.0],
            width: 1.0,
            dashed: false,
        }
    }
}

impl ShapeSegment {
    fn new(x: f32, y: f32) -> Self {
        ShapeSegment {
            x,
            y,
            style: None,
            event_id: None,
        }
    }
}

impl ElemBuider {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        ElemBuider {
            id: COUNTER.fetch_add(1, Ordering::Relaxed) * 80,
            shape: vec![
                ShapeSegment::new(0.0, 0.0),
                ShapeSegment::new(0.0, 1.0),
                ShapeSegment::new(1.0, 1.0),
                ShapeSegment::new(1.0, 0.0),
            ],
            style: LineStyle::default(),
            blur: false,
            bgcolor: [0.3, 0.3, 0.3, 1.0],
            x,
            y,
            width: w,
            height: h,
            draggable: false,
        }
    }

    pub fn with_shape(&mut self, shape: &[ShapeSegment]) -> &mut Self {
        self.shape = Vec::from(shape);
        self
    }

    pub fn with_line_style(&mut self, line_style: &LineStyle) -> &mut Self {
        self.style = *line_style;
        self
    }

    pub fn blur_on(&mut self) -> &mut Self {
        self.blur = true;
        self
    }

    pub fn with_background(&mut self, color: &[f32; 4]) -> &mut Self {
        self.bgcolor = *color;
        self
    }

    pub fn draggable(&mut self) -> &mut Self {
        self.draggable = true;
        self
    }

    pub fn build(&self) -> Element {
        Element {
            id: self.id,
            shape: self.shape.clone(),
            style: self.style,
            blur: self.blur,
            bgcolor: self.bgcolor,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            draggable: if self.draggable {
                Some(DraggableElement::default())
            } else {
                None
            },
            children_elems: Vec::new(),
        }
    }
}

impl RenderableElement for Element {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_shape(&self) -> &[ShapeSegment] {
        &self.shape
    }

    fn get_style(&self) -> &LineStyle {
        &self.style
    }

    fn is_blur(&self) -> bool {
        self.blur
    }

    fn get_bg_color(&self) -> [f32; 4] {
        self.bgcolor
    }

    fn get_position(&self) -> (i32, i32) {
        if self.draggable.is_some() {
            (
                self.x + self.draggable.as_ref().unwrap().drag_x,
                self.y + self.draggable.as_ref().unwrap().drag_y,
            )
        } else {
            (self.x, self.y)
        }
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
