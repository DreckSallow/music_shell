use std::{
    io::{self, Stdout},
    time::Duration,
};

use ratatui::{
    self,
    prelude::{Constraint, CrosstermBackend, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TuiApp {
    pub fn build() -> io::Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self { terminal })
    }

    fn setup_terminal(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        Ok(())
    }
    fn restore_terminal(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(self.terminal.show_cursor()?)
    }

    pub fn run(mut self) -> io::Result<()> {
        self.setup_terminal()?;
        loop {
            self.terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(ratatui::prelude::Direction::Vertical)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                    .split(frame.size());

                let search_block = Block::default().title("Search").borders(Borders::ALL);
                frame.render_widget(search_block, chunks[0]);

                let content_block = Block::default().title("Content").borders(Borders::ALL);
                frame.render_widget(content_block, chunks[1]);
            })?;

            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if KeyCode::Char('q') == key.code {
                        break;
                    }
                }
            }
        }
        self.restore_terminal()?;
        Ok(())
    }
}
