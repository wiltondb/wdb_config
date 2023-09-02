
use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;
//use postgres::{Client, NoTls};

use crate::*;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use notice::SyncNoticeSender;
use about_dialog_ui::AboutDialogUi;

#[derive(Default)]
pub struct AboutDialog {
    notice_sender: Option<SyncNoticeSender>,
    pub ui: AboutDialogUi,
}

impl AboutDialog {


}

impl PopupDialog<()> for AboutDialog {
    fn popup(notice_sender: SyncNoticeSender) -> JoinHandle<()> {
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