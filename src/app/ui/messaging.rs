use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use web_sys::console;
use crate::animation::Animator;
use crate::element::{ElemBuilder, Element};
use derivative::Derivative;
use crate::FieldSelector;

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
pub type HandlerCallbackMut = RefCell<Box<dyn Fn(&Msg, &mut dyn HandleContext) -> bool + 'static>>;
pub type MappingFunction = Box<dyn Fn(&FieldSelector) -> Option<FieldSelector>>;

/// This is an interface to process

pub trait HandleContext {
    fn add_bind(&mut self, source_id: usize, target_id: usize, map_fn: MappingFunction);
    fn start_animation(&self, a: Box<dyn Animator>, on_finish: Option<HandlerCallback>);
    fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback);
    fn remove_handler(&mut self, target_id: usize, message_type: Msg);
    fn add_element(&mut self, elem: Element, parent_id: usize) -> Option<usize>;
    fn remove_element(&mut self, target_id: usize);
    fn set(&self, target_id :usize, value: &FieldSelector);
}

pub(super) struct StoredAnimation {
    pub(super) animator: Box<dyn Animator>,
    pub(super) on_finish: Option<HandlerCallback>,
}

pub(super) struct HandlersBean {
    pub(super) elements: Vec<RefCell<Element>>,
    pub(super) animations: RefCell<Vec<StoredAnimation>>,
    dep_links: HashMap<usize, (usize, MappingFunction)>,
    elem_handlers: HashMap<EvtKey, HandlerCallback>,
}

impl Default for HandlersBean {
    fn default() -> Self {
        HandlersBean{
            animations: RefCell::new(Vec::new()),
            elem_handlers: HashMap::new(),
            elements: Vec::new(),
            dep_links: HashMap::new(),
        }
    }
}

impl HandlersBean {
    pub(super) fn new(w: u32, h: u32) -> Self {
        HandlersBean{
            animations: RefCell::new(Vec::new()),
            elem_handlers: HashMap::new(),
            elements: vec![RefCell::new(ElemBuilder::new(0, 0, w, h).build())],
            dep_links: HashMap::new(),
        }
    }

    pub fn remove_finished_animations(&mut self) {
        {
            let anims = self.animations.borrow();
            let handlers: Vec<&HandlerCallback> = anims.iter().filter(|anim| anim.animator.is_finished()).map(|anim| anim.on_finish.as_ref())
                .filter(|f| f.is_some()).map(|f| f.unwrap()).collect();

            for fun in handlers {
                fun.borrow_mut()(&Msg::AdvanceClock(0.0), self);
            }
        }

        self.animations.borrow_mut().retain(|animation| {
            !animation.animator.is_finished()
        });
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
    fn add_bind(&mut self, source_id: usize, target_id: usize, map_fn: MappingFunction) {
        self.dep_links.insert(source_id, (target_id, map_fn));
    }

    fn start_animation(&self, a: Box<dyn Animator>, on_finish: Option<HandlerCallback>) {
        console::log_1(&format!("Starting animation").into());
        self.animations.borrow_mut().push(StoredAnimation{
            animator: a,
            on_finish
        });
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
            let rem_ref = &to_remove;
            self.animations.borrow_mut().retain(|a| !rem_ref.contains(&a.animator.get_target()));
            self.dep_links.retain(|from, (to, _fun)| !rem_ref.contains(from) && !rem_ref.contains(to));
        }

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

    fn set(&self, target_id: usize, value: &FieldSelector) {
        self.elements[target_id].borrow_mut().set(*value);
        let mut notif_stack = Vec::new();
        notif_stack.push((target_id, value.clone()));

        while !notif_stack.is_empty() {
            let (next, val) = notif_stack.pop().unwrap();
            self.elements[next].borrow_mut().set(val);
            for (id, func) in self.dep_links.get(&next) {
                let res = func(&val);
                if res.is_some() {
                    let val = res.unwrap().clone();
                    notif_stack.push((*id, val))
                }
            }
        }
    }
}