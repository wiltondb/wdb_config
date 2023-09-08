
use super::*;

#[derive(Default)]
pub struct ConnectCheckDialogArgs {
    pub(super) notice_sender:  RefCell<ui::SyncNoticeSender>,
    pub(super) config: ConnectConfig,
}

impl ConnectCheckDialogArgs {
    pub fn new(notice: &ui::SyncNotice, config: ConnectConfig) -> Self {
        Self {
            notice_sender: RefCell::new(notice.sender()),
            config,
        }
    }

    pub fn send_notice(&self) {
        self.notice_sender.borrow().send()
    }
}

impl ui::PopupArgs for ConnectCheckDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.borrow().send()
    }
}
