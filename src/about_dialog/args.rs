
use std::cell::RefCell;

use crate::*;
use nwg_ui as ui;

#[derive(Default)]
pub struct AboutDialogArgs {
    notice_sender: RefCell<ui::SyncNoticeSender>
}

impl AboutDialogArgs {
    pub fn new(notice: &ui::SyncNotice) -> Self {
        Self {
            notice_sender: RefCell::new(notice.sender())
        }
    }
}

impl ui::PopupArgs for AboutDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.borrow().send()
    }
}

