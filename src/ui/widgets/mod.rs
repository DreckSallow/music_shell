use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;

use crate::FrameType;

pub mod list;
pub mod table;

pub trait Component {
    fn render(&mut self, frame: &mut FrameType, area: Rect);
    fn on_event(&mut self, event: &KeyEvent);
    fn is_focus(&self) -> bool {
        false
    }
}
