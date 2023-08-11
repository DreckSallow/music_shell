use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table as TableUi},
};

use crate::{
    ui::widgets::{table::Table, Component},
    FrameType,
};

pub struct Playlist<'a> {
    table: Table<'a>,
    pub is_focus: bool,
}

impl<'a> Playlist<'a> {
    pub fn new(songs: &[Vec<&'a str>]) -> Self {
        Self {
            table: Table::new(songs, &["Name", "Path", "Ext"]),
            is_focus: false,
        }
    }
}

impl<'a> Component for Playlist<'a> {
    fn render(&mut self, frame: &mut FrameType, area: Rect) {
        let playlist_block = Block::default().title("List").borders(Borders::ALL);

        let headers_cells = self.table.headers.iter().map(|header| Cell::from(*header));
        let header = Row::new(headers_cells)
            .height(1)
            .style(Style::default().fg(ratatui::style::Color::Blue));
        // .bottom_margin(1);

        let items = self.table.items.iter().map(|item| {
            let cells = item.iter().map(|text| Cell::from(*text));
            Row::new(cells).height(1)
        });
        let table_block = TableUi::new(items)
            .header(header)
            .block(playlist_block)
            .highlight_style(Style::default().bg(ratatui::style::Color::Cyan))
            .highlight_symbol(">")
            .widths(&[
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .highlight_symbol("🎵 ");
        frame.render_stateful_widget(table_block, area, self.table.state())
    }
    fn on_event(&mut self, event: &KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }
        match event.code {
            KeyCode::Down => self.table.next(),
            KeyCode::Up => self.table.previous(),
            _ => {}
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
