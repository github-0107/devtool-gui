use crate::ui::json_fmt_ui::JsonFmtUI;
use crate::ui::md5_ui::Md5UI;
use crate::ui::sql_fmt_ui::SqlFmtUI;
use fltk::{prelude::*, *};

mod ui {
    fl2rust_macro::include_ui!("devapp/ui/main_ui.fl");
}

pub struct MainUI {
    ui: ui::UserInterface,
}

impl MainUI {
    pub fn make_window() -> Self {
        let ui = ui::UserInterface::make_window();
        Self { ui }
    }

    pub fn ui(&mut self) {
        let mut main_win = self.get_window();
        self.jsn_fmt_btn_callback();
        self.md5_btn_callback();
        self.sql_fmt_btn_callback();
        main_win.end();
        main_win.show();
    }

    fn get_window(&self) -> window::DoubleWindow {
        self.ui.main_win.clone()
    }

    fn jsn_fmt_btn_callback(&mut self) {
        self.ui.jsn_fmt_btn.set_callback(|_| {
            let mut ui = JsonFmtUI::make_window();
            ui.set_ui();
        });
    }

    fn md5_btn_callback(&mut self) {
        self.ui.md5_btn.set_callback(|_| {
            let mut ui = Md5UI::make_window();
            ui.set_ui();
        });
    }

    fn sql_fmt_btn_callback(&mut self) {
        self.ui.sql_fmt_btn.set_callback(|_| {
            SqlFmtUI::make_window().set_ui();
        });
    }
}
