use std::{
    io::{self, Stdout},
    time::Duration,
};

use ratatui::{self, prelude::CrosstermBackend, Terminal};

use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{application::Application, ui::widgets::Component};

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
        let mut application = Application::new();
        loop {
            self.terminal.draw(|frame| {
                application.render(frame, frame.size());
            })?;
            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if KeyCode::Char('q') == key.code {
                        break;
                    }
                    application.on_event(&key);
                }
            }
        }
        self.restore_terminal()?;
        Ok(())
    }
}
