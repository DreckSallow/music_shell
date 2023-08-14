use ratatui::widgets::ListState;

use crate::ui::controllers::list::ListController;

pub struct List<'a> {
    pub items: Vec<&'a str>,
    controller: ListController,
}

impl<'a> List<'a> {
    pub fn new(items: &[&'a str]) -> Self {
        let index_list = if items.len() > 1 { Some(0) } else { None };
        Self {
            items: items.to_vec(),
            controller: ListController::default().with_select(index_list),
        }
    }
    pub fn state(&mut self) -> &mut ListState {
        self.controller.state()
    }
    pub fn next(&mut self) {
        self.controller.next(self.items.len());
    }

    pub fn previous(&mut self) {
        self.controller.previous(self.items.len())
    }
}
