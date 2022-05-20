use std::cell::RefCell;
use std::rc::Rc;

mod store;
pub use self::store::*;

mod assets;
pub use self::assets::*;

pub mod ui;
pub mod bluetooth;

pub use self::ui::*;

/// Used to instantiate our application
pub struct App {
    pub store: Rc<RefCell<Store>>,
}

impl App {
    /// Create a new instance of our WebGL Water application
    pub fn new(w: i32, h: i32, dw: i32, dh: i32) -> App {

        App {
            store: Rc::new(RefCell::new(Store::new(w, h, dw, dh))),
        }
    }

}
