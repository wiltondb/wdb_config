
use std::cell::RefCell;
use std::thread;
use std::thread::JoinHandle;

use nwg::NativeUi;

use crate::*;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use dialogs::PopupDialogArgs;
use notice::SyncNotice;
use notice::SyncNoticeSender;
use about_dialog_ui::AboutDialogUi;

#[derive(Default)]
pub struct AboutDialogArgs {
    notice_sender: RefCell<SyncNoticeSender>
}

impl AboutDialogArgs {
    pub fn new(notice: &SyncNotice) -> Self {
        Self {
            notice_sender: RefCell::new(notice.sender())
        }
    }
}

impl PopupDialogArgs for AboutDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.borrow().send()
    }
}

#[derive(Default)]
pub struct AboutDialog {
    args: AboutDialogArgs,
    ui: AboutDialogUi,
}

impl AboutDialog {
}

impl PopupDialog<AboutDialogUi, AboutDialogArgs, ()> for AboutDialog {
    fn popup(args: AboutDialogArgs) -> JoinHandle<()> {
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
        self.ui.hide_window();
        nwg::stop_thread_dispatch();
    }

    fn ui(&self) -> &AboutDialogUi {
        &self.ui
    }

    fn ui_mut(&mut self) -> &mut AboutDialogUi {
        &mut self.ui
    }
}