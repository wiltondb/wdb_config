
use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;
use postgres::config::Config;

use crate::*;
use dialogs::DialogJoiner;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use notice::SyncNoticeSender;
use connect_check_dialog::ConnectCheckDialog;
use connect_dialog_ui::ConnectDialogUi;

#[derive(Default)]
pub struct ConnectDialog {
    notice_sender: Option<SyncNoticeSender>,
    pub ui: ConnectDialogUi,
    check_dialog_joiner: DialogJoiner<bool>,
}

impl ConnectDialog {
    pub fn open_check_dialog(&self) {
        self.ui.window().set_enabled(false);
        let join_handle = ConnectCheckDialog::popup(self.ui.check_dialog_notice.sender());
        self.check_dialog_joiner.set_join_handle(join_handle);
    }

    pub fn await_check_dialog(&self) {
        self.ui.window().set_enabled(true);
        self.ui.check_dialog_notice.receive();
        let _ = self.check_dialog_joiner.await_result();
        //self.ui.status_bar.set_text(0, &res);
    }
}

impl PopupDialog<Config> for ConnectDialog {
    fn popup(notice_sender: SyncNoticeSender) -> JoinHandle<Config> {
        thread::spawn(move || {
            let data = Self {
                notice_sender: Some(notice_sender),
                ..Default::default()
            };
            let dialog = Self::build_ui(data).expect("Failed to build UI");
            nwg::dispatch_thread_events();
            dialog.result()
        })
    }

    fn close(&self) {
        self.notice_sender.as_ref().expect("Notice sender not initialized").send();
        self.ui.window().set_visible(false);
        nwg::stop_thread_dispatch();
    }
}
