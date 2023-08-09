use crate::FrameType;

pub mod table;

pub trait Component {
    fn render(&mut self, frame: FrameType);
    fn on_event(&mut self, event: usize);
}
