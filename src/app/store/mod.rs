use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use self::camera::*;
use self::mouse::*;
use crate::messaging::Msg;
use crate::app::ui::messaging::EventTarget;
use crate::timedata::HrmData;

mod camera;
mod mouse;
pub mod timedata;

pub struct Store {
    pub state: StateWrapper,
}

impl Store {
    pub fn new(w: i32, h: i32, dw: i32, dh: i32) -> Store {
        Store {
            state: StateWrapper(State::new(w, h, dw, dh)),
        }
    }
}

impl EventTarget for Store {
    fn msg(&mut self, msg: &Msg) -> bool {
        match msg {
            _ => self.state.msg(msg),
        }
    }
}

pub struct State {
    clock: f32,
    camera: Camera,
    mouse: Mouse,
    show_scenery: bool,
    c_width: i32,
    c_height: i32,
    d_width: i32,
    d_height: i32,
    show_pick: bool,
    hr_data: Rc<RefCell<HrmData>>,
}

impl State {
    fn new(w: i32, h: i32, dw: i32, dh: i32) -> State {
        State {
            /// Time elapsed since the application started, in milliseconds
            clock: 0.,
            camera: Camera::new((w as f32) / (h as f32)),
            mouse: Mouse::default(),
            show_scenery: true,
            c_width: w,
            c_height: h,
            d_width: dw,
            d_height: dh,
            show_pick: false,
            hr_data: Rc::new(RefCell::new( HrmData {
                data : Vec::new()
            })),
        }
    }

    pub fn viewport_width(&self) -> i32 {
        self.c_width
    }

    pub fn viewport_height(&self) -> i32 {
        self.c_height
    }

    pub fn width_rate(&self) -> f32 {
        self.c_width as f32 / self.d_width as f32
    }

    pub fn height_rate(&self) -> f32 {
        self.c_height as f32 / self.d_height as f32
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// The current time in milliseconds
    pub fn clock(&self) -> f32 {
        self.clock
    }

    pub fn show_scenery(&self) -> bool {
        self.show_scenery
    }

    pub fn show_pick(&self) -> bool {
        self.show_pick
    }

    pub fn get_hr_data(&self) -> Rc<RefCell<HrmData>> {
        self.hr_data.clone()
    }

    pub fn msg(&mut self, msg: &Msg) -> bool {
        match msg {
            Msg::AdvanceClock(dt) => {
                self.clock += dt;
                false
            }
            Msg::MouseDown(x, y) => {
                self.mouse.set_pressed(true);
                self.mouse.set_pos(*x, *y);
                false
            }
            Msg::MouseUp(_x, _y) => {
                self.mouse.set_pressed(false);
                false
            }
            Msg::MouseMove(x, y) => {
                if !self.mouse.get_pressed() {
                    return false;
                }

                let (old_x, old_y) = self.mouse.get_pos();

                let x_delta = old_x as i32 - x;
                let y_delta = y - old_y as i32;

                self.camera.orbit_left_right(x_delta as f32 / 50.0);
                self.camera.orbit_up_down(y_delta as f32 / 50.0);

                self.mouse.set_pos(*x, *y);
                false
            }
            Msg::Zoom(zoom) => {
                self.camera.zoom(*zoom);
                false
            }
            Msg::ShowScenery(show_scenery) => {
                self.show_scenery = *show_scenery;
                false
            }
            Msg::ResizeViewport(w, h) => {
                self.c_width = *w;
                self.c_height = *h;
                self.camera.update_aspect((*w as f32) / (*h as f32));
                false
            }
            Msg::KeyDown(key) => {
                if *key == 82 { //'R'
                    self.show_pick = true;
                }
                false
            }
            Msg::KeyUp(key) => {
                if *key == 82 { //'R'
                    self.show_pick = false;
                }
                false
            }
            _ => false,
        }
    }
}

pub struct StateWrapper(State);

impl Deref for StateWrapper {
    type Target = State;

    fn deref(&self) -> &State {
        &self.0
    }
}

impl StateWrapper {
    pub fn msg(&mut self, msg: &Msg) -> bool {
        self.0.msg(msg)
    }
}
