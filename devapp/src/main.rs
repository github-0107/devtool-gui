mod my_app;
mod ui;

fn main() {
    let app = my_app::MyApp::default();
    app.run().unwrap();
}
