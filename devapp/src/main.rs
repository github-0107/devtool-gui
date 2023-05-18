#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod my_app;
mod ui;

fn main() {
    let app = my_app::MyApp::default();
    app.run().unwrap();
}
