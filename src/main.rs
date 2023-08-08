mod application;
mod tui_app;

fn main() {
    let res = tui_app::TuiApp::build().unwrap().run();
    println!("Result: {:?}", res);
}
