
use std::cell::RefCell;
use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;

use crate::*;
use dialogs::DialogJoiner;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use dialogs::PopupDialogArgs;
use notice::SyncNotice;
use notice::SyncNoticeSender;
use connect_check_dialog::ConnectCheckDialog;
use connect_check_dialog::ConnectCheckDialogArgs;
use connect_check_dialog::ConnectCheckDialogResult;
use connect_dialog_ui::ConnectDialogUi;

#[derive(Default, Debug, Clone)]
pub struct ConnectConfig {
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub enable_tls: bool,
    pub accept_invalid_tls: bool,
}

#[derive(Default)]
pub struct ConnectDialogArgs {
    notice_sender: RefCell<SyncNoticeSender>,
    config: ConnectConfig,
}

impl ConnectDialogArgs {
    pub fn new(notice: &SyncNotice, config: ConnectConfig) -> Self {
        Self {
            notice_sender: RefCell::new(notice.sender()),
            config,
        }
    }
}

impl PopupDialogArgs for ConnectDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.borrow().send()
    }
}

#[derive(Default)]
pub struct ConnectDialog {
    args: ConnectDialogArgs,
    ui: ConnectDialogUi,
    check_dialog_joiner: DialogJoiner<ConnectCheckDialogResult>,
}

impl ConnectDialog {
    pub fn open_check_dialog(&self) {
        self.ui.window().set_enabled(false);
        let notice = self.ui.check_dialog_notice();
        let config = self.ui.config_from_input();
        let args = ConnectCheckDialogArgs::new(notice, config);
        let join_handle = ConnectCheckDialog::popup(args);
        self.check_dialog_joiner.set_join_handle(join_handle);
    }

    pub fn await_check_dialog(&self) {
        self.ui.window().set_enabled(true);
        self.ui.check_dialog_notice().receive();
        let _ = self.check_dialog_joiner.await_result();
        dialogs::shake_window(self.ui.window());
    }

    pub fn on_port_input_changed(&self) {
        self.ui.correct_port_value();
    }

    pub fn on_enable_tls_checkbox_changed(&self) {
        self.ui.sync_tls_checkboxes_state();
    }
}

impl PopupDialog<ConnectDialogUi, ConnectDialogArgs, ConnectConfig> for ConnectDialog {
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
        self.ui.window().set_visible(false);
        nwg::stop_thread_dispatch();
    }

    fn ui(&self) -> &ConnectDialogUi {
        &self.ui
    }

    fn ui_mut(&mut self) -> &mut ConnectDialogUi {
        &mut self.ui
    }

    fn result(&self) -> ConnectConfig {
        self.ui.config_from_input()
    }
}
