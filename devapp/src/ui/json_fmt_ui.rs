use devtool::json_fmt;
use fltk::{prelude::*, text::TextEditor, *};

mod ui {
    fl2rust_macro::include_ui!("devapp/ui/json_fmt_ui.fl");
}

pub struct JsonFmtUI {
    ui: ui::UserInterface,
}

impl JsonFmtUI {
    pub fn make_window() -> Self {
        let ui = ui::UserInterface::make_window();
        Self { ui }
    }

    pub fn set_ui(&mut self) {
        let mut win = self.ui.json_fmt_win.clone();
        let buffer = text::TextBuffer::default();
        let mut text_editor = self.ui.json_text.clone();
        text_editor.set_size(530, 350);
        text_editor.set_buffer(buffer);
        self.submit_btn_callback(text_editor.clone());
        win.end();
        win.show();
    }

    fn submit_btn_callback(&mut self, text_editor: TextEditor) {
        self.ui.submit.set_callback(move |_| {
            if let Some(mut text_buffer) = text_editor.buffer() {
                let text = text_buffer.text();
                let fmt = json_fmt::Formatter::default().with(&text);
                match fmt.pretty() {
                    Ok(json) => text_buffer.set_text(json.as_str()),
                    Err(e) => dialog::message(550, 450, e.to_string().as_str()),
                }
            }
        });
    }
}
