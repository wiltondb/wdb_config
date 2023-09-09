
use super::*;

#[derive(Default)]
pub struct ConnectDialog {
    pub(super) c: ConnectDialogControls,

    args: ConnectDialogArgs,
    check_joiner: ui::PopupJoinHandle<ConnectCheckDialogResult>,
}

impl ConnectDialog {
    pub fn open_check_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(false);
        let config = self.config_from_input();
        let args = ConnectCheckDialogArgs::new(&self.c.check_notice, config);
        self.check_joiner = ConnectCheckDialog::popup(args);
    }

    pub fn await_check_dialog(&mut self, _: nwg::EventData) {
        self.c.window.set_enabled(true);
        self.c.check_notice.receive();
        let _ = self.check_joiner.join();
        self.c.shake_window();
    }

    pub fn on_port_input_changed(&mut self, _: nwg::EventData) {
        self.correct_port_value();
    }

    pub fn on_enable_tls_checkbox_changed(&mut self, _: nwg::EventData) {
        self.sync_tls_checkboxes_state();
    }

    pub fn correct_port_value(&self) {
        let text = self.c.port_input.text();
        if text.len() == 0 {
            self.c.port_input.set_text("1");
            return;
        }
        let num = match text.parse::<u128>() {
            Err(_) => {
                self.c.port_input.set_text("5432");
                return;
            },
            Ok(n) => n
        };
        if num > 65535 {
            self.c.port_input.set_text("65535");
        }
    }

    pub fn config_from_input(&self) -> ConnectConfig {
        let port = match self.c.port_input.text().parse::<u16>() {
            Ok(n) => n,
            Err(_) => 5432,
        };
        ConnectConfig {
            hostname: self.c.hostname_input.text(),
            port,
            username: self.c.username_input.text(),
            password: self.c.password_input.text(),
            enable_tls: self.c.enable_tls_checkbox.check_state() == nwg::CheckBoxState::Checked,
            accept_invalid_tls: self.c.enable_tls_checkbox.enabled() &&
                self.c.accept_invalid_tls_checkbox.check_state() == nwg::CheckBoxState::Checked
        }
    }

    pub fn config_to_input(&self, config: &ConnectConfig) {
        self.c.hostname_input.set_text(&config.hostname);
        self.c.port_input.set_text(&config.port.to_string());
        self.c.username_input.set_text(&config.username);
        self.c.password_input.set_text(&config.password);
        let tls_state = if config.enable_tls {
            nwg::CheckBoxState::Checked
        } else {
            nwg::CheckBoxState::Unchecked
        };
        self.c.enable_tls_checkbox.set_check_state(tls_state);
        let accept_state = if config.accept_invalid_tls {
            nwg::CheckBoxState::Checked
        } else {
            nwg::CheckBoxState::Unchecked
        };
        self.c.accept_invalid_tls_checkbox.set_check_state(accept_state);
    }

    pub fn sync_tls_checkboxes_state(&self) {
        let enabled = self.c.enable_tls_checkbox.check_state() == nwg::CheckBoxState::Checked;
        self.c.accept_invalid_tls_checkbox.set_enabled(enabled);
    }
}

impl ui::PopupDialog<ConnectDialogArgs, ConnectConfig> for ConnectDialog {
    fn popup(args: ConnectDialogArgs) -> ui::PopupJoinHandle<ConnectConfig> {
        let join_handle = thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let mut dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        });
        ui::PopupJoinHandle::from(join_handle)
    }

    fn init(&mut self) {
        self.config_to_input(&self.args.config);
        self.c.shake_window();
    }

    fn result(&mut self) -> ConnectConfig {
        self.config_from_input()
    }

    fn close(&mut self, _: nwg::EventData) {
        self.args.notify_parent();
        self.c.hide_window();
        nwg::stop_thread_dispatch();
    }
}
