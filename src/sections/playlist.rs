use std::path::PathBuf;

use ratatui::{
    prelude::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table as TableUi},
};

use crate::{ui::widgets::table::Table, FrameType};

#[derive(Clone, Debug)]
pub struct Song {
    name: String,
    path: PathBuf,
    ext: String,
}

impl Song {
    pub fn new<P: Into<PathBuf>>(name: &str, path: P, ext: &str) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            ext: ext.into(),
        }
    }
    pub fn get_info_string(&self) -> [&str; 3] {
        return [
            self.name.as_str(),
            self.path.to_str().unwrap_or("---"),
            self.ext.as_str(),
        ];
    }
}

pub struct Playlist<'a> {
    // songs: Vec<[&'a str; 3]>,
    table: Table<'a>,
}

impl<'a> Playlist<'a> {
    pub fn new(songs: &[Song]) -> Self {
        // let sgn = Song::new("TQG", r"C:\Users\DIKSON\personal\music_shell", "mp3");
        // let songs = [sgn.clone(), sgn.clone(), sgn.clone(), sgn.clone()];
        let song = ["TQG", r"C:\Users\DIKSON\personal\music_shell", "mp3"].to_vec();
        let songs = [song.clone(), song.clone()];
        let table = Table::new(&songs, &["Name", "Path", "Ext"]);
        // println!("table headers: {:?}", table.headers);
        Self {
            // songs: songs.to_vec(),
            table,
        }
    }
    pub fn render(&mut self, frame: &mut FrameType, area: Rect) {
        let playlist_block = Block::default().title("List").borders(Borders::ALL);

        let headers_cells = self.table.headers.iter().map(|header| Cell::from(*header));
        let header = Row::new(headers_cells)
            .height(1)
            .style(Style::default().fg(ratatui::style::Color::Blue))
            .bottom_margin(1);

        let items = self.table.items.iter().map(|item| {
            let cells = item.iter().map(|text| Cell::from(*text));
            Row::new(cells).height(2)
        });
        let table_block = TableUi::new(items)
            .header(header)
            .block(playlist_block)
            .highlight_style(Style::default().bg(ratatui::style::Color::Green))
            .highlight_symbol(">")
            .widths(&[
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]);
        // .highlight_symbol("👉");
        frame.render_widget(table_block, area)
        // frame.render_stateful_widget(table_block, area, self.table.state())
    }
}
