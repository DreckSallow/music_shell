use ratatui::widgets::TableState;

#[derive(Default)]
pub struct TableController {
    state: TableState,
}

impl TableController {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
        }
    }
    pub fn state(&mut self) -> &mut TableState {
        &mut self.state
    }

    pub fn next(&mut self, len: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, len: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
