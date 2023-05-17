use devtool::md5::Encrypter;
use fltk::{input::Input, prelude::*};

mod ui {
    fl2rust_macro::include_ui!("devapp/ui/md5_ui.fl");
}

pub struct Md5UI {
    ui: ui::UserInterface,
}

#[derive(Clone)]
struct Inputer {
    ipt: Input,
    up32: Input,
    lw32: Input,
    up16: Input,
    lw16: Input,
}

impl Md5UI {
    pub fn make_window() -> Self {
        let ui = ui::UserInterface::make_window();
        Self { ui }
    }

    pub fn set_ui(&mut self) {
        let mut win = self.ui.md5_win.clone();
        let ipter = self.get_inputer();
        self.encrypt_btn_callback(ipter.clone());
        self.clear_btn_callback(ipter);
        win.end();
        win.show();
    }

    fn get_inputer(&self) -> Inputer {
        let up32 = self.ui.upper32_input.clone();
        let lw32 = self.ui.lower32_input.clone();
        let up16 = self.ui.upper16_input.clone();
        let lw16 = self.ui.lower16_input.clone();
        let ipt = self.ui.input_text.clone();
        Inputer {
            ipt,
            up32,
            lw32,
            up16,
            lw16,
        }
    }

    fn encrypt_btn_callback(&mut self, mut ipter: Inputer) {
        self.ui.encrypt_btn.set_callback(move |_| {
            let value = ipter.ipt.value();
            let encrypter = Encrypter::default().with(value);
            let (upper32, lower32, upper16, lower16) = encrypter.encrypt();
            ipter.up32.set_value(upper32.as_str());
            ipter.lw32.set_value(lower32.as_str());
            ipter.up16.set_value(upper16.as_str());
            ipter.lw16.set_value(lower16.as_str());
        });
    }

    fn clear_btn_callback(&mut self, mut ipter: Inputer) {
        self.ui.clear_btn.set_callback(move |_| {
            ipter.ipt.set_value("");
            ipter.up32.set_value("");
            ipter.lw32.set_value("");
            ipter.up16.set_value("");
            ipter.lw16.set_value("");
        });
    }
}
