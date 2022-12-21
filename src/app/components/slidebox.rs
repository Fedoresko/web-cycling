use std::sync::atomic::{AtomicI32, Ordering};
use crate::components::{Component, UserEvent};
use crate::{ElemBuilder, FieldSelector, LineStyle, log, Msg, ShapeSegment, Vec4};
use crate::HandlerImpact::Set;
use crate::messaging::HandlersBean;

pub struct SlideBox {
    root: usize,
    edge: usize,
    handle: usize,
}

impl SlideBox {
    pub fn new() -> SlideBox {
        SlideBox {
            root: 0,
            edge: 0,
            handle: 0,
        }
    }
}

const EDGE_SIZE : i32 = 6;

static S_Y: AtomicI32 = AtomicI32::new(0);
static S_X: AtomicI32 = AtomicI32::new(0);
static S_HEIGHT: AtomicI32 = AtomicI32::new(100);
static S_WIDTH: AtomicI32 = AtomicI32::new(100);
static P_HEIGHT: AtomicI32 = AtomicI32::new(0);
static T : AtomicI32 = AtomicI32::new(-1);

impl Component for SlideBox {
    fn initialize(&mut self, parent: usize, ui: &mut HandlersBean) -> usize {
        let root = ElemBuilder::new(0,0,100,100).with_line_style(&LineStyle{
            color: [0.2,0.2,0.2,1.0],
            dashed: false,
            width: 1.0,
        }).filled_rect(&[0.0,0.0,0.0,0.9]).build();
        self.root = ui.add_element(root, parent).unwrap();

        let edge = ElemBuilder::new(0,100, 100, EDGE_SIZE).filled_rect(&[0.5,0.5,0.5,1.0])
            .with_gradient(2,vec![1.0,0.0], vec![Vec4::from([0.5,0.5,0.5,1.0]), Vec4::from([0.0,0.0,0.0,1.0])],
        (0.0,0.0), (0.0,1.0)).draggable().build();
        self.edge = ui.add_element(edge, self.root).unwrap();


        log!("Slidebox initialized {} edge {}", self.root, self.edge);

        let handle = ElemBuilder::new(30, 100 + EDGE_SIZE as i32, 60, 16)
            .svg("ArrowDown").build();
        self.handle = ui.add_element(handle, self.root).unwrap();

        ui.add_bind(self.root, self.edge, Box::new(|fs| {
            if let FieldSelector::X(x) = *fs {
                return Some(vec![FieldSelector::X(x)]);
            } else if let FieldSelector::Y(y) = *fs {
                S_Y.store(y, Ordering::Relaxed);
                return Some(vec![FieldSelector::Y(y + S_HEIGHT.load(Ordering::Relaxed))]);
            } else if let FieldSelector::Height(h) = *fs {
                S_HEIGHT.store(h, Ordering::Relaxed);
                return Some(vec![FieldSelector::Y(h + S_Y.load(Ordering::Relaxed))]);
            } else if let FieldSelector::Width(w) = *fs {
                return Some(vec![FieldSelector::Width(w)]);
            }
            None
        }));

        ui.add_bind(self.root, self.handle, Box::new(
            |fs| {
                if let FieldSelector::X(x) = *fs {
                    S_X.store(x, Ordering::Relaxed);
                    return Some(vec![FieldSelector::X(S_WIDTH.load(Ordering::Relaxed) as i32/2 - 30 + x)]);
                } else if let FieldSelector::Y(y) = *fs {
                    S_Y.store(y, Ordering::Relaxed);
                    return Some(vec![FieldSelector::Y(S_HEIGHT.load(Ordering::Relaxed) as i32 + EDGE_SIZE + y)]);
                } else if let FieldSelector::Height(h) = *fs {
                    return Some(vec![FieldSelector::Y(h as i32 + EDGE_SIZE + S_Y.load(Ordering::Relaxed))]);
                } else if let FieldSelector::Width(w) = *fs {
                    S_WIDTH.store(w, Ordering::Relaxed);
                    return Some(vec![FieldSelector::X(w as i32/2 - 30 + S_X.load(Ordering::Relaxed))]);
                }
                None
            }
        ));

        let root_id = self.root;
        ui.register_handler(self.handle, Msg::MouseDown(0,0), Box::new(move |ms| {
            log!("slidebox mouse down");
            let height = S_HEIGHT.load(Ordering::Relaxed);
            let new_height =
            if height != 0 {
                P_HEIGHT.store(height, Ordering::Relaxed);
                0
            } else {
                P_HEIGHT.load(Ordering::Relaxed)
            };
            let res = FieldSelector::Height(new_height);
            Set(root_id, res)
        }));

        self.root
    }


    fn handle(&mut self, event: &UserEvent, ui: &HandlersBean) -> Option<Vec<UserEvent>> {
        if let UserEvent::ProcessDrag((id, x,y)) = event {
            T.compare_exchange(-1, S_HEIGHT.load(Ordering::Relaxed), Ordering::Relaxed, Ordering::Relaxed).ok();
            log!("Dragging {} ({}, {})",id,x,y);
            if *id == self.edge {
                ui.set(self.root, FieldSelector::Height(*y + T.load(Ordering::Relaxed)));
            }
        } else if let UserEvent::ProcessDrop((_id, _x, _y)) = event {
            T.store(-1, Ordering::Relaxed);
        }
        None
    }
}