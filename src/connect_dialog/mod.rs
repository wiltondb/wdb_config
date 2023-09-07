

mod args;
mod config;
mod controls;
mod events;
mod layout;
mod nui;

use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;

use crate::*;
use nwg_ui as ui;
use ui::Controls;
use ui::PopupArgs;
use ui::PopupDialog;
use connect_check_dialog::ConnectCheckDialog;
use connect_check_dialog::ConnectCheckDialogArgs;
use connect_check_dialog::ConnectCheckDialogResult;

pub use config::ConnectConfig;
pub use args::ConnectDialogArgs;
pub(self) use controls::ConnectDialogControls;
use events::ConnectDialogEvents;
use layout::ConnectDialogLayout;

#[derive(Default)]
pub struct ConnectDialog {
    pub(self) controls: ConnectDialogControls,
    pub(self) layout: ConnectDialogLayout,
    pub(self) events: ConnectDialogEvents,

    args: ConnectDialogArgs,
    check_joiner: ui::PopupJoiner<ConnectCheckDialogResult>,
}

impl ConnectDialog {
    pub fn open_check_dialog(&self) {
        self.controls.window.set_enabled(false);
        let config = self.config_from_input();
        let args = ConnectCheckDialogArgs::new(&self.controls.check_notice, config);
        let join_handle = ConnectCheckDialog::popup(args);
        self.check_joiner.set_join_handle(join_handle);
    }

    pub fn await_check_dialog(&self) {
        self.controls.window.set_enabled(true);
        self.controls.check_notice.receive();
        let _ = self.check_joiner.await_result();
        self.controls.shake_window();
    }

    pub fn on_port_input_changed(&self) {
        self.correct_port_value();
    }

    pub fn on_enable_tls_checkbox_changed(&self) {
        self.sync_tls_checkboxes_state();
    }


    pub fn correct_port_value(&self) {
        let text = self.controls.port_input.text();
        if text.len() == 0 {
            self.controls.port_input.set_text("1");
            return;
        }
        let num = match text.parse::<u128>() {
            Err(_) => {
                self.controls.port_input.set_text("5432");
                return;
            },
            Ok(n) => n
        };
        if num > 65535 {
            self.controls.port_input.set_text("65535");
        }
    }

    pub fn config_from_input(&self) -> ConnectConfig {
        let port = match self.controls.port_input.text().parse::<u16>() {
            Ok(n) => n,
            Err(_) => 5432,
        };
        ConnectConfig {
            hostname: self.controls.hostname_input.text(),
            port,
            username: self.controls.username_input.text(),
            password: self.controls.password_input.text(),
            enable_tls: self.controls.enable_tls_checkbox.check_state() == nwg::CheckBoxState::Checked,
            accept_invalid_tls: self.controls.enable_tls_checkbox.enabled() &&
                self.controls.accept_invalid_tls_checkbox.check_state() == nwg::CheckBoxState::Checked
        }
    }

    pub fn sync_tls_checkboxes_state(&self) {
        let enabled = self.controls.enable_tls_checkbox.check_state() == nwg::CheckBoxState::Checked;
        self.controls.accept_invalid_tls_checkbox.set_enabled(enabled);
    }
}

impl ui::PopupDialog<ConnectDialogArgs, ConnectConfig> for ConnectDialog {
    fn popup(args: ConnectDialogArgs) -> JoinHandle<ConnectConfig> {
        thread::spawn(move || {
            let data = Self {
                args,
                ..Default::default()
            };
            let dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        })
    }

    fn close(&self) {
        self.args.notify_parent();
        self.controls.hide_window();
        nwg::stop_thread_dispatch();
    }

    fn result(&self) -> ConnectConfig {
        self.config_from_input()
    }
}
