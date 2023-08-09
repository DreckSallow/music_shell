use ratatui::{
    prelude::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};

use crate::{
    sections::{library::PlaylistLibrary, playlist::Playlist},
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
    pub fn render(&mut self, frame: &mut FrameType) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(frame.size());

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        self.library_section.render(frame, content_chunks[0]);
        self.playlist_section.render(frame, content_chunks[1]);

        let content_block = Block::default().title("Player").borders(Borders::ALL);
        frame.render_widget(content_block, chunks[1]);
    }
}
