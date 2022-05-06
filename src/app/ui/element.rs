use std::sync::atomic::{AtomicUsize, Ordering};

use crate::app::ui::drag::DraggableElement;
use crate::app::ui::render::RenderableElement;
use crate::fields::{FieldSelector, Vec4};

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
    id: usize,
    shape: Vec<ShapeSegment>,
    style: LineStyle,
    blur: bool,
    bgcolor: Vec4,
    pub (super) x: i32,
    pub (super) y: i32,
    width: u32,
    height: u32,
    pub (super) children_elems: Vec<usize>,
    pub (super) parent_element: usize,
    pub (super) draggable: Option<DraggableElement>,
    gradient_stops: u8,
    gradient_pos: Option<Vec<f32>>,
    gradient_colors: Option<Vec<Vec4>>,
    gradient_start: Option<(f32, f32)>,
    gradient_end: Option<(f32, f32)>,
}

pub struct ElemBuider {
    id: usize,
    shape: Vec<ShapeSegment>,
    style: LineStyle,
    blur: bool,
    bgcolor: Vec4,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    draggable: bool,
    gradient_stops: u8,
    gradient_pos: Option<Vec<f32>>,
    gradient_colors: Option<Vec<Vec4>>,
    gradient_start: Option<(f32, f32)>,
    gradient_end: Option<(f32, f32)>,
}

impl Element  {
    pub fn children(&self) -> &[usize] {
        self.children_elems.as_slice()
    }

    pub (super) fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn set(&mut self, field: FieldSelector) {
        match field {
            FieldSelector::X(value) => { self.x = value; }
            FieldSelector::Y(value) => { self.y = value; }
            FieldSelector::Width(value) => { self.width = value; }
            FieldSelector::Height(value) => { self.height = value; }
            FieldSelector::BGColor(value) => { self.bgcolor = value; }
            FieldSelector::GradientPos0(value) => { self.gradient_pos.as_mut().unwrap()[0] = value; }
            FieldSelector::GradientColors0(value) => { self.gradient_colors.as_mut().unwrap()[0] = value; }
            FieldSelector::GradientPos1(value) => { self.gradient_pos.as_mut().unwrap()[1] = value; }
            FieldSelector::GradientColors1(value) => { self.gradient_colors.as_mut().unwrap()[1] = value; }
            FieldSelector::GradientPos2(value) => { self.gradient_pos.as_mut().unwrap()[2] = value; }
            FieldSelector::GradientColors2(value) => { self.gradient_colors.as_mut().unwrap()[2] = value; }
            FieldSelector::GradientPos3(value) => { self.gradient_pos.as_mut().unwrap()[3] = value; }
            FieldSelector::GradientColors3(value) => { self.gradient_colors.as_mut().unwrap()[3] = value; }
            FieldSelector::GradientStart(value) => { self.gradient_start = Some( value ); }
            FieldSelector::GradientEnd(value) => { self.gradient_end = Some( value ); }
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
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
        ElemBuider {
            id: 0,
            shape: vec![
                ShapeSegment::new(0.0, 0.0),
                ShapeSegment::new(0.0, 1.0),
                ShapeSegment::new(1.0, 1.0),
                ShapeSegment::new(1.0, 0.0),
                ShapeSegment::new(0.0, 0.0),
            ],
            style: LineStyle::default(),
            blur: false,
            bgcolor: Vec4::from([0.0, 0.0, 0.0, 0.0]),
            x,
            y,
            width: w,
            height: h,
            draggable: false,
            gradient_stops: 0,
            gradient_end: None,
            gradient_start: None,
            gradient_pos: None,
            gradient_colors: None,
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
        self.bgcolor = Vec4::from(*color);
        self
    }

    pub fn draggable(&mut self) -> &mut Self {
        self.draggable = true;
        self
    }

    pub fn with_gradient(&mut self, n :u8, pos: Vec<f32>, colors: Vec<Vec4>, start: (f32, f32), end: (f32, f32)) ->  &mut Self {
        if n < 2 || n > 10 {
            panic!("Number of stops must be in rage 2-10");
        }

        self.gradient_stops = n;
        self.gradient_pos = Some(pos);
        self.gradient_colors = Some(colors);
        self.gradient_start = Some(start);
        self.gradient_end = Some(end);
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
            parent_element: 0,
            children_elems: Vec::new(),
            gradient_stops: self.gradient_stops,
            gradient_end: self.gradient_end,
            gradient_start: self.gradient_start,
            gradient_pos: self.gradient_pos.clone(),
            gradient_colors: self.gradient_colors.clone(),
        }
    }
}

impl RenderableElement for Element {
    fn get_id(&self) -> usize {
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
        self.bgcolor.into()
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

    fn get_gradient_stops_n(&self) -> u8 {
        self.gradient_stops
    }

    fn get_gradient_positions(&self) -> &[f32] {
        self.gradient_pos.as_ref().unwrap().as_slice()
    }

    fn get_gradient_colors(&self) -> &[Vec4] {
        self.gradient_colors.as_ref().unwrap().as_slice()
    }

    fn get_gradient_start(&self) -> (f32, f32) {
        self.gradient_start.unwrap()
    }

    fn get_gradient_end(&self) -> (f32, f32) {
        self.gradient_end.unwrap()
    }
}
