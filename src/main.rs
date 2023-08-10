use std::io::Stdout;

use ratatui::{prelude::CrosstermBackend, Frame};

mod application;
mod sections;
mod tui_app;
mod ui;

pub(crate) type FrameType<'a> = Frame<'a, CrosstermBackend<Stdout>>;

fn main() {
    let res = tui_app::TuiApp::build().unwrap().run();
    println!("Result: {:?}", res);
}
