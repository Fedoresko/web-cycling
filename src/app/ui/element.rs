use std::cell::Cell;
use crate::app::ui::drag::DraggableElement;
use crate::app::ui::render::RenderableElement;
use crate::fields::{FieldSelector, Vec4};
use crate::text::RenderableString;

#[derive(Clone, Copy, Debug)]
pub struct LineStyle {
    pub color: [f32; 4],
    pub width: f32,
    pub dashed: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ShapeSegment {
    pub x: f32,
    pub y: f32,
    pub style: Option<LineStyle>,
    pub event_id: Option<i32>,
}

#[derive(Debug)]
pub struct Element {
    pub (super) id: usize,
    shape: Vec<ShapeSegment>,
    style: Option<LineStyle>,
    blur: bool,
    bgcolor: Vec4,
    pub (super) x: i32,
    pub (super) y: i32,
    width: i32,
    height: i32,
    pub (super) children_elems: Vec<usize>,
    pub (super) parent_element: usize,
    pub (super) draggable: Option<DraggableElement>,
    gradient_stops: u8,
    gradient_pos: Option<Vec<f32>>,
    gradient_colors: Option<Vec<Vec4>>,
    gradient_start: Option<(f32, f32)>,
    gradient_end: Option<(f32, f32)>,
    svg: Option<String>,
    label: Option<RenderableString>,
    pub(super) direct_drag: bool,
}

pub struct ElemBuilder {
    id: usize,
    shape: Vec<ShapeSegment>,
    style: Option<LineStyle>,
    blur: bool,
    bgcolor: Vec4,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    draggable: bool,
    gradient_stops: u8,
    gradient_pos: Option<Vec<f32>>,
    gradient_colors: Option<Vec<Vec4>>,
    gradient_start: Option<(f32, f32)>,
    gradient_end: Option<(f32, f32)>,
    svg: Option<String>,
    label: Option<RenderableString>,
    direct_drag: bool,
}

pub trait UINode {
    fn children(&self) -> &[usize];
    fn set_id(&mut self, id: usize);
    fn set(&mut self, field: FieldSelector);
    fn get_id(&self) -> usize;
    fn get_parent(&self) -> usize;
}

impl Element {
    pub fn get_svg(&self) -> &Option<String> {
        &self.svg
    }
}

impl UINode for Element  {
    fn children(&self) -> &[usize] {
        self.children_elems.as_slice()
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn set(&mut self, field: FieldSelector) {
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
            FieldSelector::LabelText(value) => { let s : String = value.iter().collect();  self.label.as_mut().unwrap().string = String::from(s.trim()); }
            FieldSelector::LabelColor(value) => { self.label.as_mut().unwrap().color = value; }
            FieldSelector::None => {}
        }
   }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_parent(&self) -> usize {
        self.parent_element
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

#[allow(dead_code)]
impl ElemBuilder {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        ElemBuilder {
            id: 0,
            shape: Vec::new(),
            style: None,
            blur: false,
            bgcolor: Vec4::from([0.0, 0.0, 0.0, 1.0]),
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
            svg: None,
            label: None,
            direct_drag: false,
        }
    }

    pub fn with_shape(&mut self, shape: &[ShapeSegment]) -> &mut Self {
        self.shape = Vec::from(shape);
        self
    }

    pub fn filled_rect(&mut self,  color: &[f32; 4]) -> &mut Self {
        self.bgcolor = Vec4::from(*color);
        self.shape = vec![
            ShapeSegment::new(0.0, 0.0),
            ShapeSegment::new(0.0, 1.0),
            ShapeSegment::new(1.0, 1.0),
            ShapeSegment::new(1.0, 0.0),
            ShapeSegment::new(0.0, 0.0),
        ];
        self
    }

    pub fn with_label(&mut self, value: &str, font : &str, size: f32, color: Vec4) -> &mut Self  {
        self.label = Some(RenderableString{
            string : String::from(value),
            font_size: size,
            color,
            font: String::from(font),
            pos: Cell::new((0, 0)),
        });
        self
    }

    pub fn with_line_style(&mut self, line_style: &LineStyle) -> &mut Self {
        self.style = Some(*line_style);
        self
    }

    pub fn blur_on(&mut self) -> &mut Self {
        self.blur = true;
        self
    }

    pub fn direct_drag(&mut self) -> &mut Self {
        self.direct_drag = true;
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

    pub fn svg(&mut self, svg: &str) -> &mut Self {
        self.svg = Some(String::from(svg));
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
        let elem = Element {
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
            svg: self.svg.clone(),
            label: self.label.clone(),
            direct_drag: self.direct_drag,
        };
        elem
    }
}

impl RenderableElement for Element {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_shape(&self) -> &[ShapeSegment] {
        &self.shape
    }

    fn get_style(&self) -> Option<&LineStyle> {
        self.style.as_ref()
    }

    fn is_blur(&self) -> bool {
        self.blur
    }

    fn get_bg_color(&self) -> [f32; 4] {
        self.bgcolor.into()
    }

    fn get_position(&self) -> (i32, i32) {
        if self.draggable.is_some() && self.direct_drag {
            (
                self.x + self.draggable.as_ref().unwrap().drag_x,
                self.y + self.draggable.as_ref().unwrap().drag_y,
            )
        } else {
            (self.x, self.y)
        }
    }

    fn get_size(&self) -> (i32, i32) {
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

    fn get_svg(&self) -> &Option<String> {
        &self.svg
    }

    fn get_label(&self) -> &Option<RenderableString> {
        &self.label
    }

    fn get_opacity(&self) -> f32 {
        self.bgcolor[3]
    }
}
