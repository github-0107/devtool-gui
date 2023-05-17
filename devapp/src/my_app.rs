use crate::ui::main_ui::MainUI;
use fltk::{app::App, prelude::FltkError};

#[derive(Default)]
pub struct MyApp {
    app: App,
}

impl MyApp {
    pub fn run(&self) -> Result<(), FltkError> {
        MainUI::make_window().ui();
        self.app.run()
    }
}
