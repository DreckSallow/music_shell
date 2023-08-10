use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
};

use crossterm::event::KeyEvent;

use crate::{
    sections::{library::PlaylistLibrary, playlist::Playlist},
    ui::widgets::Component,
    FrameType,
};

pub struct Application<'a> {
    library_section: PlaylistLibrary<'a>,
    playlist_section: Playlist<'a>,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        Application {
            library_section: PlaylistLibrary::new(vec![]),
            playlist_section: Playlist::new(&[]),
        }
    }
}

impl<'a> Component for Application<'a> {
    fn render(&mut self, frame: &mut FrameType, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(area);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        self.library_section.render(frame, content_chunks[0]);
        self.playlist_section.render(frame, content_chunks[1]);

        let content_block = Block::default().title("Player").borders(Borders::ALL);
        frame.render_widget(content_block, chunks[1]);
    }

    fn on_event(&mut self, event: &KeyEvent) {
        self.playlist_section.on_event(event);
    }
}
