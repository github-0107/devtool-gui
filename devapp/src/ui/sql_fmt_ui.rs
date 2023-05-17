use devtool::sql_fmt::SqlFormater;
use fltk::{button::CheckButton, input::Input, prelude::*, text::TextEditor, *};

mod ui {
    fl2rust_macro::include_ui!("devapp/ui/sql_fmt_ui.fl");
}

pub struct SqlFmtUI {
    ui: ui::UserInterface,
}

#[derive(Clone, Debug)]
struct Widget {
    text_editor: TextEditor,
    uppercase: CheckButton,
    space: Input,
}

impl SqlFmtUI {
    pub fn make_window() -> Self {
        let ui = ui::UserInterface::make_window();
        Self { ui }
    }

    pub fn set_ui(&mut self) {
        let mut win = self.ui.sql_fmt_win.clone();
        let buffer = text::TextBuffer::default();
        let mut widget = self.get_widget();
        widget.text_editor.set_size(540, 280);
        widget.text_editor.set_buffer(buffer);
        widget.space.set_value("2");
        self.fmt_btn_callback(widget.clone());
        self.clear_btn_callback(widget.clone());
        win.end();
        win.show();
    }

    fn get_widget(&self) -> Widget {
        let text_editor = self.ui.text_editor.clone();
        let uppercase = self.ui.uppercase.clone();
        let space = self.ui.space.clone();
        Widget {
            text_editor,
            uppercase,
            space,
        }
    }

    fn fmt_btn_callback(&mut self, widget: Widget) {
        self.ui.fmt_btn.set_callback(move |_| {
            if let Some(mut text_buffer) = widget.text_editor.buffer() {
                let text = text_buffer.text();
                let is_upper = widget.uppercase.is_checked();
                let space = widget.space.value();
                let space = space.parse::<u8>().unwrap_or(2);
                let fmt = SqlFormater::default().with(&text, space, is_upper);
                text_buffer.set_text(fmt.format().as_str());
            }
        });
    }

    fn clear_btn_callback(&mut self, widget: Widget) {
        self.ui.clear_btn.set_callback(move |_| {
            if let Some(mut text_buffer) = widget.text_editor.buffer() {
                text_buffer.set_text("");
            }
        });
    }
}
