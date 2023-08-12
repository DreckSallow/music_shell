use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List as ListUi, ListItem},
};

use crate::{
    ui::widgets::{list::List, Component},
    FrameType,
};

pub struct PlaylistLibrary<'a> {
    list: List<'a>,
    pub is_focus: bool,
}

impl<'a> PlaylistLibrary<'a> {
    pub fn new(playlists: Vec<&'a str>) -> Self {
        let songs = &["Rap", "Pop", "Lofi"];
        Self {
            list: List::new(songs),
            is_focus: false,
        }
    }
}

impl<'a> Component for PlaylistLibrary<'a> {
    fn render(&mut self, frame: &mut FrameType, area: Rect) {
        let styled = if self.is_focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };
        let section = Block::default()
            .title("Playlist")
            .borders(Borders::ALL)
            .border_style(styled);
        let items: Vec<ListItem> = self
            .list
            .items
            .iter()
            .map(|itm| ListItem::new(*itm))
            .collect();

        let list_block = ListUi::new(items)
            .block(section)
            .highlight_style(Style::default().bg(Color::Cyan))
            .highlight_symbol("🚀 ");
        frame.render_stateful_widget(list_block, area, self.list.state())
    }
    fn on_event(&mut self, event: &crossterm::event::KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }
        match event.code {
            KeyCode::Down => self.list.next(),
            KeyCode::Up => self.list.previous(),
            _ => {}
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
