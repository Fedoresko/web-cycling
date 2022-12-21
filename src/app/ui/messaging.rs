use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use web_sys::console;
use crate::animation::Animator;
use crate::element::{ElemBuilder, Element, UINode};
use derivative::Derivative;
use multimap::MultiMap;
use crate::{FieldSelector, WebEventDispatcher};
use crate::components::{Component, UserEvent};

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
    AnimationFinished,
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

pub type HandlerCallback = Box<dyn Fn(&Msg) -> HandlerImpact + 'static>;
//pub type HandlerCallbackMut = RefCell<Box<dyn Fn(&Msg) -> HandlerImpact + 'static>>;
pub type MappingFunction = Box<dyn Fn(&FieldSelector) -> Option<Vec<FieldSelector>>>;

/// This is an interface to process

#[allow(dead_code)]
pub enum HandlerImpact {
    AddBind(usize, usize, MappingFunction),
    StartAnimation(Box<dyn Animator>, Option<HandlerCallback>),
    RegisterHandler(usize, Msg, HandlerCallback),
    RemoveHandler(usize, Msg),
    AddElement(Element, usize),
    RemoveElement(usize),
    Set(usize, FieldSelector),
    None
}

// pub trait HandleContext {
//     fn add_bind(&mut self, source_id: usize, target_id: usize, map_fn: MappingFunction);
//     fn start_animation(&self, a: Box<dyn Animator>, on_finish: Option<HandlerCallback>);
//     fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback);
//     fn remove_handler(&mut self, target_id: usize, message_type: Msg);
//     fn add_element(&mut self, elem: Element, parent_id: usize) -> Option<usize>;
//     fn remove_element(&mut self, target_id: usize);
//     fn set(&self, target_id :usize, value: &FieldSelector);
// }

pub struct StoredAnimation {
    pub(super) animator: Box<dyn Animator>,
    pub(super) on_finish: Option<HandlerCallback>,
}

pub struct HandlersBean {
    pub(super) elements: Vec<RefCell<Element>>,
    pub(super) animations: RefCell<Vec<StoredAnimation>>,
    dep_links: MultiMap<usize, (usize, MappingFunction)>,
    elem_handlers: HashMap<EvtKey, HandlerCallback>,
    pub(super) components: Vec<RefCell<Box<dyn Component>>>,
    event_queue: Vec<UserEvent>,
}

impl Default for HandlersBean {
    fn default() -> Self {
        HandlersBean{
            animations: RefCell::new(Vec::new()),
            elem_handlers: HashMap::new(),
            elements: Vec::new(),
            dep_links: MultiMap::new(),
            components: Vec::new(),
            event_queue: Vec::new(),
        }
    }
}

impl HandlersBean {
    pub(super) fn new(w: i32, h: i32) -> Self {
        HandlersBean{
            animations: RefCell::new(Vec::new()),
            elem_handlers: HashMap::new(),
            elements: vec![RefCell::new(ElemBuilder::new(0, 0, w, h).build())],
            dep_links: MultiMap::new(),
            components: Vec::new(),
            event_queue: Vec::new(),
        }
    }

    pub fn remove_finished_animations(&mut self) {
        {
           let mut impacts = Vec::new();

            {
                let anims = self.animations.borrow();
                let handlers: Vec<&HandlerCallback> = anims.iter().filter(|anim| anim.animator.is_finished()).map(|anim| anim.on_finish.as_ref())
                    .filter(|f| f.is_some()).map(|f| f.unwrap()).collect();

                for fun in handlers {
                    impacts.push(fun(&Msg::AnimationFinished));
                }
            }

            for impact in impacts {
                self.process_impact(impact)
            }
        }

        self.animations.borrow_mut().retain(|animation| {
            !animation.animator.is_finished()
        });
    }

    pub fn get_handler(&self, target_id : usize, message_type: Msg) -> Option<&HandlerCallback> {
        self.elem_handlers.get(&EvtKey { target_id, message_type })
    }

    fn collect_children(&self, id : usize) -> Vec<usize> {
        let mut res = Vec::new();
        for child in self.elements.get( self.get_elem_pos(id)).unwrap().borrow().children() {
            res.push(*child);
            res.extend(self.collect_children(*child).iter());
        };
        res
    }

    pub fn process_impact(&mut self, impact: HandlerImpact) {
        match impact {
            HandlerImpact::AddBind(source_id, target_id, map_fn) => { self.add_bind(source_id, target_id, map_fn); }
            HandlerImpact::StartAnimation(animator, on_finish) => { self.start_animation(animator, on_finish); }
            HandlerImpact::RegisterHandler(target_id, message_type, callback) => { self.register_handler(target_id, message_type, callback); }
            HandlerImpact::RemoveHandler(target_id, message_type) => { self.remove_handler(target_id, message_type); }
            HandlerImpact::AddElement(elem, parent_id) => { self.add_element(elem, parent_id); }
            HandlerImpact::RemoveElement(target_id) => { self.remove_element(target_id); }
            HandlerImpact::Set(target_id, value) => { self.set(target_id, value); }
            HandlerImpact::None => {}
        }
    }

    pub fn add_bind(&mut self, source_id: usize, target_id: usize, map_fn: MappingFunction) {
        self.dep_links.insert(source_id, (target_id, map_fn));
        console::log_1(&"After bind".into());
        let arr : Vec<String> = self.dep_links.iter_all().map(|e| format!("{} to {}",e.0, e.1.len())).collect();
        let s = "dep_links [".to_owned()+arr.join(", ").as_str()+"]";
        console::log_1(&s.into());
    }

    pub fn start_animation(&self, a: Box<dyn Animator>, on_finish: Option<HandlerCallback>) {
        console::log_1(&format!("Starting animation").into());
        self.animations.borrow_mut().push(StoredAnimation{
            animator: a,
            on_finish
        });
    }

    pub fn register_handler(&mut self, target_id: usize, message_type: Msg, callback: HandlerCallback) {
        console::log_1(&format!("Registered handler {} {}", target_id, message_type).into());
        self.elem_handlers.insert(EvtKey { target_id, message_type }, callback);
    }

    pub fn remove_handler(&mut self, target_id: usize, message_type: Msg) {
        self.elem_handlers.remove(&EvtKey { target_id, message_type });
    }

    pub fn add_element(&mut self, elem: Element, parent_id: usize)  -> Option<usize> {
        let mut element = elem;
        let pos = self.elements.len();
        element.set_id(pos);
        self.elements.get(parent_id)?.borrow_mut().children_elems.push(pos);
        element.parent_element = parent_id;
        self.elements.push(RefCell::new(element));
        Some(pos)
    }

    pub fn remove_element(&mut self, target_id: usize) {
        console::log_1(&"Before removal".into());
        let arr : Vec<String> = self.dep_links.iter_all().map(|e| format!("{} to {}",e.0, e.1.len())).collect();
        let s = "dep_links [".to_owned()+arr.join(", ").as_str()+"]";
        console::log_1(&s.into());

        let arr : Vec<String> = self.elements.iter().map(|e| format!("elem: {} children {:?}",e.borrow().get_id(), e.borrow().children() )).collect();
        let s = "elements [".to_owned()+arr.join(", ").as_str()+"]";
        console::log_1(&s.into());


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
            let id = element.borrow().get_id();
            let t = element.borrow().children().iter().map(|e| *move_ref.get(e).unwrap_or(e) ).collect();
            element.borrow_mut().children_elems = t;
            //element.borrow_mut().set_id(*move_ref.get(&id).unwrap_or(&id));
        }

        // for mut dlink in &mut self.dep_links {
        //     let t = &mut dlink.1.0;
        //     if let Some(to) = move_ref.get(t) {
        //         *t = *to;
        //     }
        // }

        console::log_1(&"After removal".into());
        let arr : Vec<String> = self.dep_links.iter_all().map(|e| format!("{} to {}",e.0, e.1.len())).collect();
        let s = "dep_links [".to_owned()+arr.join(", ").as_str()+"]";
        console::log_1(&s.into());

        let arr : Vec<String> = self.elements.iter().map(|e| format!("elem: {} children {:?}",e.borrow().get_id(), e.borrow().children() )).collect();
        let s = "elements [".to_owned()+arr.join(", ").as_str()+"]";
        console::log_1(&s.into());

    }

    pub fn set(&self, target_id: usize, value: FieldSelector) {
        let id = self.get_elem_pos(target_id);

        self.elements[id].borrow_mut().set(value);
        let mut notif_stack = Vec::new();
        notif_stack.push((target_id, value.clone()));

        while !notif_stack.is_empty() {
            let (next, val) = notif_stack.pop().unwrap();
            let pos = self.get_elem_pos(next);
            self.elements[pos].borrow_mut().set(val);
            if let Some(links) = self.dep_links.get_vec(&next) {
                for (id, func) in links {
                    let res = func(&val);
                    if res.is_some() {
                        let val = res.unwrap().clone();
                        for selector in val {
                            notif_stack.push((*id, selector))
                        }
                    }
                }
            }
        }
    }

    pub(super) fn elem_by_id(&self, target_id : usize) -> &RefCell<Element> {
        let pos = self.get_elem_pos(target_id);
        self.elements.get(pos).unwrap()
    }

    fn get_elem_pos(&self, target_id: usize) -> usize {
        let mut id = target_id;
        for (k, e) in self.elements.iter().enumerate() {
            if e.borrow().get_id() == target_id {
                id = k;
            }
        }
        id
    }

    pub (super) fn add_component<T>(&mut self, mut component: T, parent: usize) -> usize where T : Component + 'static {
        let root = component.initialize(parent, self);
        self.components.push(RefCell::new(Box::new(component)));
        root
    }

    pub (super) fn process_events(&mut self) {
        let mut new_events = Vec::new();
        for event in self.event_queue.iter() {
            for component in self.components.iter() {
                if let Some(evts) = component.borrow_mut().handle(event, self).as_mut() {
                    new_events.append(evts);
                }
            }
        }
        self.event_queue = new_events;
    }

    pub fn push_event(&mut self, event: UserEvent) {
        self.event_queue.push(event);
    }
}

impl EventTarget for HandlersBean {
    fn msg(&mut self, msg: &Msg) -> bool {
        false
    }
}

pub trait EventTarget {
    fn msg(&mut self, msg: &Msg) -> bool;
}

impl EventTarget for WebEventDispatcher {
    fn msg(&mut self, msg: &Msg) -> bool {
        if !self.ui.msg(msg) {
            self.app.store.borrow_mut().msg(msg);
        }
        true
    }
}
