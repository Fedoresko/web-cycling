use crate::messaging::HandlersBean;

pub mod hrm_display;
pub mod slidebox;

#[derive(Copy, Clone, Debug)]
pub enum UserEvent {
    HrChanged(i32),
    ProcessDrag((usize, i32, i32)),
    ProcessDrop((usize, i32, i32)),
    Clicked(usize),
}

pub trait Component {
    /// Init elements and return main element id
    fn initialize(&mut self, parent: usize, ui: &mut HandlersBean) -> usize;
    /// Consume event and produce new
    fn handle(&mut self, event: &UserEvent, ui: &HandlersBean) -> Option<Vec<UserEvent>>;
}