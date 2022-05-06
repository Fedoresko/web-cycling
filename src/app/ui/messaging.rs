use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use web_sys::console;
use crate::animation::Animator;
use crate::element::{ElemBuilder, Element};
use derivative::Derivative;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Derivative)]
#[derivative(Eq, PartialEq, Hash)]
pub enum Msg {
    AdvanceClock(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        f32),
    MouseDown(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    MouseUp(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    MouseMove(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    Zoom(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]f32),
    ShowScenery(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]bool),
    KeyDown(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]u32),
    KeyUp(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]u32),
    ResizeViewport(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    DragEvent(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
    DropEvent(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32,
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]i32),
}

impl Display for Msg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct EvtKey {
    target_id: usize,
    message_type: Msg,
}

pub type HandlerCallback = RefCell<Box<dyn Fn(&Msg, &dyn HandleContext) -> bool + 'static>>;

/// This is an interface to process

pub trait HandleContext {
    fn start_animation(&self, a: Box<dyn Animator>);
    fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback);
    fn remove_handler(&mut self, target_id: usize, message_type: Msg);
    fn add_element(&mut self, elem: Element, parent_id: usize) -> Option<usize>;
    fn remove_element(&mut self, target_id: usize);
}

pub(super) struct HandlersBean {
    pub(super) elements: Vec<RefCell<Element>>,
    pub(super) animations: RefCell<Vec<Box<dyn Animator>>>,
    elem_handlers: HashMap<EvtKey, HandlerCallback>,
}

impl Default for HandlersBean {
    fn default() -> Self {
        HandlersBean{
            animations: RefCell::new(Vec::new()),
            elem_handlers: HashMap::new(),
            elements: Vec::new(),
        }
    }
}

impl HandlersBean {
    pub(super) fn new(w: u32, h: u32) -> Self {
        HandlersBean{
            animations: RefCell::new(Vec::new()),
            elem_handlers: HashMap::new(),
            elements: vec![RefCell::new(ElemBuilder::new(0, 0, w, h).build())],
        }
    }

    pub(super) fn get_handler(&self, target_id : usize, message_type: Msg) -> Option<&HandlerCallback> {
        self.elem_handlers.get(&EvtKey { target_id, message_type })
    }

    fn collect_children(&self, id : usize) -> Vec<usize> {
        let mut res = Vec::new();
        for child in self.elements.get(id).unwrap().borrow().children() {
            res.push(*child);
            res.extend(self.collect_children(*child).iter());
        };
        res
    }
}

impl HandleContext for HandlersBean {
    fn start_animation(&self, animation: Box<dyn Animator>) {
        console::log_1(&format!("Starting animation").into());
        self.animations.borrow_mut().push(animation);
    }

    fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback) {
        console::log_1(&format!("Registered handler {} {}", target_id, message_type).into());
        self.elem_handlers.insert(EvtKey { target_id, message_type }, callback);
    }

    fn remove_handler(&mut self, target_id: usize, message_type: Msg) {
        self.elem_handlers.remove(&EvtKey { target_id, message_type });
    }

    fn add_element(&mut self, elem: Element, parent_id: usize)  -> Option<usize> {
        let mut element = elem;
        let pos = self.elements.len();
        element.set_id(pos);
        self.elements.get(parent_id)?.borrow_mut().children_elems.push(pos);
        element.parent_element = parent_id;
        self.elements.push(RefCell::new(element));
        Some(pos)
    }

    fn remove_element(&mut self, target_id: usize) {
        let mut to_remove = HashSet::new();
        to_remove.insert(target_id);
        to_remove.extend(self.collect_children(target_id));

        {
            let parent = self.elements.remove(target_id).borrow().parent_element;
            self.elements.get(parent).unwrap().borrow_mut().children_elems.retain(|e| *e != target_id);
        }

        {
            let rem_ref = &to_remove;
            self.elements.retain(|e| !rem_ref.contains(&e.borrow().get_id()));
        }

        let mut move_map = HashMap::new();
        for (pos, element) in self.elements.iter().enumerate() {
            let ep = element.borrow().get_id();
            if ep != pos {
                move_map.insert(ep, pos);
            }
        }

        let move_ref = &move_map;
        for element in &self.elements {
            let t = element.borrow().children().iter().map(|e| *move_ref.get(e).unwrap_or(e) ).collect();
            element.borrow_mut().children_elems = t;
        }
    }
}