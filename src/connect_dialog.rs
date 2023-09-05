
use std::cell::RefCell;
use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;
use postgres::config::Config;

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

#[derive(Default)]
pub struct ConnectDialogArgs {
    notice_sender: RefCell<SyncNoticeSender>,
    config: Config,
}

impl ConnectDialogArgs {
    pub fn new(notice: &SyncNotice, config: Config) -> Self {
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
        let config = self.ui.get_input_postgres_config();
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
        self.ui.correct_port_value()
    }
}

impl PopupDialog<ConnectDialogUi, ConnectDialogArgs, Config> for ConnectDialog {
    fn popup(args: ConnectDialogArgs) -> JoinHandle<Config> {
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
}
