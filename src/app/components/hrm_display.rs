use crate::{ElemBuilder, FieldSelector, SizedStr, Sizing, Vec4};
use crate::components::{Component, UserEvent};
use crate::components::UserEvent::HrChanged;
use crate::messaging::HandlersBean;

pub struct HRMDisplay {
    root_el: usize,
    heart_img: usize,
    text: usize,
    value: u32,
}

impl Component for HRMDisplay {
    fn initialize(&mut self, parent: usize, ui: &mut HandlersBean) -> usize {
        self.create_hr_control(parent, ui)
    }

    fn handle(&mut self, event: &UserEvent, ui: &HandlersBean) -> Option<Vec<UserEvent>> {
        if let HrChanged(hr) = event {
            ui.set(self.text, &FieldSelector::LabelText(SizedStr::sizify(format!("{}", hr).as_str())))
        }
        None
    }
}

impl HRMDisplay {
    pub fn new() -> HRMDisplay {
        HRMDisplay {
            root_el: 0,
            heart_img: 0,
            text: 0,
            value: 0,
        }
    }

    fn create_hr_control(&mut self, parent: usize, ui: &mut HandlersBean) -> usize {
        let root = ElemBuilder::new(0, 0, 300, 120).build();

        self.root_el = ui.add_element(root, parent).unwrap();

        let heart = ElemBuilder::new(0, 0, 130, 120)
            .svg("HRM").build();
        self.heart_img = ui.add_element(heart, self.root_el).unwrap();


        ui.add_bind(self.root_el, self.heart_img, Box::new(|fs: &FieldSelector| {
            if let FieldSelector::Height(h) = *fs {
                return Some(vec![FieldSelector::Height(h), FieldSelector::Width(h * 11 / 10)]);
            }
            None
        }));
        ui.add_bind(self.root_el, self.heart_img, Box::new(|fs: &FieldSelector| {
            if let FieldSelector::X(x) = *fs {
                return Some(vec![FieldSelector::X(x)]);
            }
            None
        }));
        ui.add_bind(self.root_el, self.heart_img, Box::new(|fs: &FieldSelector| {
            if let FieldSelector::Y(y) = *fs {
                return Some(vec![FieldSelector::Y(y)]);
            }
            None
        }));

        let heart_rate = ElemBuilder::new(150, 0, 150, 100)
            .with_label("180", "SourceSansPro-Black", 96.0, Vec4::from([1.0, 1.0, 1.0, 1.0])).build();
        self.text = ui.add_element(heart_rate, 0).unwrap();
        ui.add_bind(self.root_el, self.text, Box::new(|fs: &FieldSelector| {
            if let FieldSelector::X(x) = *fs {
                return Some(vec![FieldSelector::X(x + 150)]);
            }
            None
        }));
        ui.add_bind(self.root_el, self.text, Box::new(|fs: &FieldSelector| {
            if let FieldSelector::Y(y) = *fs {
                return Some(vec![FieldSelector::Y(y)]);
            }
            None
        }));

        self.root_el
    }
}