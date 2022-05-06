use std::collections::HashMap;
use std::convert::TryInto;

use lyon::math::Point;
use lyon::path::PathEvent;
use lyon::tessellation::*;
use nalgebra::Vector3;
use nalgebra::Matrix3;
use usvg::{LinearGradient, NodeKind, Paint, Tree};

struct RenderablePath {
    bgcolor: [f32; 4],
    gradient_stops: u8,
    gradient_pos: Option<Vec<f32>>,
    gradient_colors: Option<Vec<[f32; 4]>>,
    gradient_start: Option<(f32, f32)>,
    gradient_end: Option<(f32, f32)>,
}

struct GpuVertex {
    position: [f32; 2],
    prim_id: u32,
}

fn load_svg(filename: &str) {
    let opt = usvg::Options::default();
    let file_data = std::fs::read(filename).unwrap();
    let rtree = Tree::from_data(&file_data, &opt.to_ref()).unwrap();

    let mut fill_tess = FillTessellator::new();
    //let mut stroke_tess = StrokeTessellator::new();

    //let view_box = rtree.svg_node().view_box;
    //let mut builder = ElemBuider::new(view_box.rect.x() as i32, view_box.rect.y() as i32, view_box.rect.width() as u32, view_box.rect.height() as u32);
    //let root = builder.with_background(&[0.0,0.0,0.0,0.0]).build();

    let mut gradients: HashMap<String, LinearGradient> = HashMap::new();

    let mut mesh: VertexBuffers<_, u32> = VertexBuffers::new();
    let mut primitives = Vec::new();

    for node in rtree.root().descendants() {
        let data = &*node.borrow();
        match data {
            NodeKind::Svg(_) => {}
            NodeKind::Defs => {}
            NodeKind::LinearGradient(gradient) => {
                gradients.insert(gradient.id.clone(), gradient.clone());
            }
            NodeKind::RadialGradient(_) => {}
            NodeKind::ClipPath(_) => {}
            NodeKind::Mask(_) => {}
            NodeKind::Pattern(_) => {}
            NodeKind::Filter(_) => {}
            NodeKind::Path(path) => {
                let t = &data.transform();
                let m: Matrix3<f32> = Matrix3::new(t.a as f32, t.c as f32, t.e as f32, t.b as f32, t.d as f32, t.f as f32, 0.0, 0.0, 1.0);
                let paint = &path.fill.as_ref().unwrap().paint;
                match paint {
                    Paint::Color(col) => {
                        primitives.push(RenderablePath {
                            bgcolor: [col.red as f32 / 256.0, col.green as f32 / 256.0, col.blue as f32 / 256.0, path.fill.as_ref().unwrap().opacity.value() as f32],
                            gradient_stops: 0,
                            gradient_colors: None,
                            gradient_pos: None,
                            gradient_start: None,
                            gradient_end: None,
                        });
                    }
                    Paint::Link(link) => {
                        let grad = gradients.get(link);
                        if grad.is_some() {
                            let g = grad.unwrap();
                            let n = g.stops.len();
                            let t = g.transform;
                            primitives.push(RenderablePath {
                                bgcolor: [1.0, 1.0, 1.0, 1.0],
                                gradient_stops: n as u8,
                                gradient_colors: Some(g.stops.iter().map(|s| [s.color.red as f32 / 256.0, s.color.green as f32 / 256.0, s.color.blue as f32 / 256.0, s.opacity.value() as f32]).collect()),
                                gradient_pos: Some(g.stops.iter().map(|s| s.offset.value() as f32).collect()),
                                gradient_start: Some(((g.x1 * t.a + g.y1 * t.c + t.e) as f32, (g.x1 * t.b + g.y1 * t.d + t.f) as f32)),
                                gradient_end: Some(((g.x2 * t.a + g.y2 * t.c + t.e) as f32, (g.x2 * t.b + g.y2 * t.d + t.f) as f32)),
                            });
                        } else {
                            primitives.push(RenderablePath {
                                bgcolor: [1.0, 1.0, 1.0, 1.0],
                                gradient_stops: 0,
                                gradient_colors: None,
                                gradient_pos: None,
                                gradient_start: None,
                                gradient_end: None,
                            });
                        }
                    }
                }

                fill_tess
                    .tessellate(
                        convert_path(&path),
                        &FillOptions::tolerance(0.01),
                        &mut BuffersBuilder::new(
                            &mut mesh,
                            VertexCtor {
                                prim_id: primitives.len() as u32 - 1,
                                transform: m,
                            },
                        ),
                    )
                    .expect("Error during tesselation!");
            }
            NodeKind::Image(_) => {}
            NodeKind::Group(_) => {}
        }
    }
}

pub struct VertexCtor {
    pub prim_id: u32,
    pub transform: Matrix3<f32>,
}

impl FillVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: FillVertex) -> GpuVertex {
        let position = vertex.position().to_array();
        let vec = self.transform.clone() * Vector3::new(position[0], position[1], 0.0);
        GpuVertex {
            position: vec.columns(0, 2).as_slice().try_into().expect(""),
            prim_id: self.prim_id,
        }
    }
}

impl StrokeVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}

fn point(x: &f64, y: &f64) -> Point {
    Point::new((*x) as f32, (*y) as f32)
}

pub struct PathConvIter<'a> {
    iter: std::slice::Iter<'a, usvg::PathSegment>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        let next = self.iter.next();
        match next {
            Some(usvg::PathSegment::MoveTo { x, y }) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = point(x, y);
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    self.first = point(x, y);
                    self.needs_end = true;
                    Some(PathEvent::Begin { at: self.first })
                }
            }
            Some(usvg::PathSegment::LineTo { x, y }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Line {
                    from,
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::CurveTo {
                     x1,
                     y1,
                     x2,
                     y2,
                     x,
                     y,
                 }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Cubic {
                    from,
                    ctrl1: point(x1, y1),
                    ctrl2: point(x2, y2),
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::ClosePath) => {
                self.needs_end = false;
                self.prev = self.first;
                Some(PathEvent::End {
                    last: self.prev,
                    first: self.first,
                    close: true,
                })
            }
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub fn convert_path(p: &usvg::Path) -> PathConvIter {
    PathConvIter {
        iter: p.data.iter(),
        first: Point::new(0.0, 0.0),
        prev: Point::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}

static FALLBACK_COLOR: usvg::Color = usvg::Color { red: 100, green: 100, blue: 100 };

pub fn convert_stroke(s: &usvg::Stroke) -> (usvg::Color, StrokeOptions) {
    let color = match s.paint {
        usvg::Paint::Color(c) => c,
        _ => FALLBACK_COLOR,
    };
    let linecap = match s.linecap {
        usvg::LineCap::Butt => LineCap::Butt,
        usvg::LineCap::Square => LineCap::Square,
        usvg::LineCap::Round => LineCap::Round,
    };
    let linejoin = match s.linejoin {
        usvg::LineJoin::Miter => LineJoin::Miter,
        usvg::LineJoin::Bevel => LineJoin::Bevel,
        usvg::LineJoin::Round => LineJoin::Round,
    };

    let opt = StrokeOptions::tolerance(0.01)
        .with_line_width(s.width.value() as f32)
        .with_line_cap(linecap)
        .with_line_join(linejoin);

    (color, opt)
}