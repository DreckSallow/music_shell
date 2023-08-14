pub mod library;
pub mod playlist;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};

use crate::ui::widgets::Component;

use self::{library::PlaylistLibrary, playlist::Playlist};

pub struct PlayerSection<'a> {
    library: PlaylistLibrary<'a>,
    playlist: Playlist<'a>,
}

impl<'a> PlayerSection<'a> {
    pub fn new() -> Self {
        let mut library = PlaylistLibrary::new(vec![]);
        library.is_focus = true;
        Self {
            library,
            playlist: Playlist::new(&[]),
        }
    }
}

impl<'a> Component for PlayerSection<'a> {
    fn render(&mut self, frame: &mut crate::FrameType, area: ratatui::prelude::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(area);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        self.library.render(frame, content_chunks[0]);
        self.playlist.render(frame, content_chunks[1]);

        let content_block = Block::default().title("Player").borders(Borders::ALL);
        frame.render_widget(content_block, chunks[1]);
    }

    fn on_event(&mut self, event: &crossterm::event::KeyEvent) {
        if let KeyModifiers::CONTROL = event.modifiers {
            match event.code {
                KeyCode::Char('2') => {
                    self.playlist.is_focus = true;
                    self.library.is_focus = false
                }
                KeyCode::Char('1') => {
                    self.library.is_focus = true;
                    self.playlist.is_focus = false
                }
                _ => {}
            }
        }

        if self.playlist.is_focus() {
            self.playlist.on_event(event);
        }
        if self.library.is_focus() {
            self.library.on_event(event);
        }
    }
}
