use std::cell::RefCell;

use crate::*;
use nwg_ui as ui;
use super::*;

#[derive(Default)]
pub struct LoadSettingsDialogArgs {
    notice_sender:  RefCell<ui::SyncNoticeSender>,
    pub(super) config: ConnectConfig,
}

impl LoadSettingsDialogArgs {
    pub fn new(notice: &ui::SyncNotice, config: ConnectConfig) -> Self {
        Self {
            notice_sender: RefCell::new(notice.sender()),
            config,
        }
    }
}

impl ui::PopupArgs for LoadSettingsDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.borrow().send()
    }
}
