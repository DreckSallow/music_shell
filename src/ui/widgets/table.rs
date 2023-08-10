use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use ratatui::widgets::TableState;

use crate::ui::controllers::table::TableController;

type TableVec<'a> = Vec<Vec<&'a str>>;

type TableSlice<'a> = [&'a str];

pub struct Table<'a> {
    pub items: TableVec<'a>,
    pub headers: Vec<&'a str>,
    controller: TableController,
}

impl<'a> Table<'a> {
    pub fn new(items: &[Vec<&'a str>], headers: &'a TableSlice) -> Self {
        Self {
            items: items.to_vec(),
            headers: headers.to_vec(),
            controller: TableController::new(0),
        }
    }
    pub fn state(&mut self) -> &mut TableState {
        self.controller.state()
    }
    pub fn next(&mut self) {
        self.controller.next(self.items.len());
    }

    pub fn previous(&mut self) {
        self.controller.previous(self.items.len())
    }
}
