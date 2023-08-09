use std::path::PathBuf;

use ratatui::{
    prelude::Rect,
    widgets::{Block, Borders},
};

struct LibrarySong<'a> {
    name: &'a str,
    path: PathBuf,
}

pub struct Playlist<'a> {
    songs: Vec<LibrarySong<'a>>,
    name: String,
}

use crate::FrameType;

pub struct PlaylistLibrary<'a> {
    playlists: Vec<Playlist<'a>>,
}

impl<'a> PlaylistLibrary<'a> {
    pub fn new(playlists: Vec<Playlist<'a>>) -> Self {
        let playlists = vec![Playlist {
            name: "Rap".into(),
            songs: vec![],
        }];
        // Self { playlists: playlists }
        Self { playlists }
    }

    pub fn render(&self, frame: &mut FrameType, area: Rect) {
        let section = Block::default().title("Playlist").borders(Borders::ALL);
        // let size=sectio
        frame.render_widget(section, area);
    }
}
