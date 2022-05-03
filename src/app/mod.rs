use std::cell::RefCell;
use std::rc::Rc;

mod store;
pub use self::store::*;

mod assets;
pub use self::assets::*;

pub mod ui;
pub use self::ui::*;

/// Used to instantiate our application
pub struct App {
    assets: Assets,
    pub store: Rc<RefCell<Store>>,
}

impl App {
    /// Create a new instance of our WebGL Water application
    pub fn new(w: i32, h: i32, dw: i32, dh: i32) -> App {
        let assets = Assets::new();

        App {
            assets,
            store: Rc::new(RefCell::new(Store::new(w, h, dw, dh))),
        }
    }

    pub fn assets(&self) -> &Assets {
        &self.assets
    }
}
