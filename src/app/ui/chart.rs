use std::rc::Rc;
use std::borrow::{Borrow, BorrowMut};
use serde::de::Unexpected::Char;
use crate::{FieldSelector, Render, State, Vec4, WebRenderer};
use crate::shader::{Shader, ShaderKind};
use web_sys::WebGl2RenderingContext as GL;
use crate::element::UINode;
use crate::timedata::TimeSeries;

pub struct Chart<T : TimeSeries<f32> > {
    sources: Vec< Rc<T>>,
    colors: Vec<Vec4>,
    widths: Vec<f32>,
    from: usize, to: usize,
    x : i32,
    y : i32,
    width: i32,
    height: i32,
    id: usize,
    parent: usize,
}

impl<T : TimeSeries<f32>> Chart<T> {
    pub fn new(data: &[Rc<T>], colors : &[Vec4], witdth : &[f32], from: usize, to: usize) -> Chart<T> {
        Chart {
            sources: Vec::from(data),
            colors: Vec::from(colors),
            widths: Vec::from(witdth),
            from,
            to,
            x: 0, y: 0, width: 100, height: 100,
            id: 0,
            parent: 0,
        }
    }

    pub fn fetch(&mut self, from: usize, to: usize) {
        self.from = from;
        self.to = to;
    }
}

impl<T : TimeSeries<f32>> UINode for Chart<T> {
    fn children(&self) -> &[usize] {
        &[]
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn set(&mut self, field: FieldSelector) {
        match field {
            FieldSelector::X(x) => { self.x = x; }
            FieldSelector::Y(y) => { self.y = y; }
            FieldSelector::Width(w) => { self.width = w; }
            FieldSelector::Height(h) => { self.height = h; }
            _ => {}
        }
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_parent(&self) -> usize {
        self.parent
    }
}

impl<T : TimeSeries<f32>> Render for Chart<T> {
    fn shader_kind(&self) -> ShaderKind {
        ShaderKind::UI
    }

    fn buffer_attributes(&self, _: &GL, _: &Shader) {

    }

    fn render(&self, gl: &GL, state: &State, shader: &Shader, _: &WebRenderer) {
        let w = gl.drawing_buffer_width() as f32;
        let h = gl.drawing_buffer_height() as f32;

        for (idx, data) in self.sources.iter().enumerate() {
            let pos_attrib = gl.get_attrib_location(&shader.program, "position");

            let step = (self.to as f32 - self.from as f32)/self.width as f32;
            let values = data.fetch_data(self.from, self.to, step);
            let values1 = data.fetch_data(self.from, self.to, step);
            let maxvalue = values.fold(-f32::INFINITY, |a : f32, b : f32| a.max(b));

            let t: Vec<f32> = values1
                .enumerate()
                .flat_map(|(i,val)| [maxvalue / val, self.width as f32 / i as f32])
                .collect();
            Self::buffer_f32_data(&gl, &t, pos_attrib as u32, 3);

            let opacity_uni = shader.get_uniform_location(gl, "opacity");
            let color_uni = shader.get_uniform_location(gl, "color");
            let blur_uni = shader.get_uniform_location(gl, "blur");
            let texrate_uni = shader.get_uniform_location(gl, "tex_rate");
            let resolution_uni = shader.get_uniform_location(gl, "iResolution");

            let stops_attrib = gl.get_uniform_location(&shader.program, "n_stops");

            gl.uniform2fv_with_f32_array(
                texrate_uni.as_ref(),
                &[state.width_rate(), state.height_rate()],
            );
            gl.uniform2fv_with_f32_array(resolution_uni.as_ref(), &[w, h]);
            gl.uniform1i(shader.get_uniform_location(gl, "iChannel0").as_ref(), 0);
            gl.uniform1f(opacity_uni.as_ref(), 1.0);

            let c = *self.colors.get(idx).unwrap();
            let v : [f32;4] = c.into();

            gl.uniform4fv_with_f32_array(color_uni.as_ref(), &v );
            gl.uniform1i(stops_attrib.as_ref(), 0);
            gl.uniform1i(blur_uni.as_ref(), 0);
            gl.line_width(*self.widths.get(idx).unwrap());

            gl.draw_arrays(GL::LINE_STRIP, 0, t.len() as i32);
        }
    }
}