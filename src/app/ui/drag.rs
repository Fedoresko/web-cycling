use crate::app::ui::Element;

pub struct DraggableElement {
    pub drag_x: i32,
    pub drag_y: i32,
}

impl Default for DraggableElement {
    fn default() -> Self {
        DraggableElement {
            drag_x: 0,
            drag_y: 0,
        }
    }
}

pub trait Draggable {
    /// supports drag
    fn is_draggable(&self) -> bool;
    /// element is being dragged
    fn process_drag(&mut self, x: i32, y: i32);
    /// element was dragged and released
    fn process_drop(&mut self);
    /// process
    fn process_cancel(&mut self);
}

impl Draggable for Element {
    fn is_draggable(&self) -> bool {
        self.draggable.is_some()
    }

    fn process_drag(&mut self, x: i32, y: i32) {
        self.draggable.as_mut().unwrap().drag_x = x;
        self.draggable.as_mut().unwrap().drag_y = y;
    }

    fn process_drop(&mut self) {
        self.x += self.draggable.as_mut().unwrap().drag_x;
        self.y += self.draggable.as_mut().unwrap().drag_y;
        self.draggable.as_mut().unwrap().drag_x = 0;
        self.draggable.as_mut().unwrap().drag_y = 0;
    }

    fn process_cancel(&mut self) {
        self.draggable.as_mut().unwrap().drag_x = 0;
        self.draggable.as_mut().unwrap().drag_y = 0;
    }
}
