use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};

use crossterm::event::KeyEvent;

use crate::{sections::player::PlayerSection, ui::widgets::Component, FrameType};

type TabsType<'a> = Vec<(&'a str, Box<dyn Component>)>;

pub struct Application<'a> {
    tabs: TabsType<'a>,
    tab_index: usize,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        let tabs: TabsType<'a> = vec![("player", Box::new(PlayerSection::new()))];
        Application { tabs, tab_index: 0 }
    }
}

impl<'a> Component for Application<'a> {
    fn render(&mut self, frame: &mut FrameType, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(area);

        let tab_titles = self.tabs.iter().map(|(tab, _)| Line::from(*tab)).collect();
        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("tabs"))
            .select(self.tab_index)
            .highlight_style(Style::default().bg(Color::Red));

        frame.render_widget(tabs, chunks[0]);

        let tab_info = self.tabs.get_mut(self.tab_index);
        if let Some((_, section)) = tab_info {
            section.render(frame, chunks[1]);
        }
        // self.player_section.render(frame, area)
    }

    fn on_event(&mut self, event: &KeyEvent) {
        let tab_info = self.tabs.get_mut(self.tab_index);
        if let Some((_, section)) = tab_info {
            section.on_event(event);
        } // self.player_section.on_event(event)
    }
}
