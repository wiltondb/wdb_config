
use super::*;

#[derive(Default)]
pub struct ConnectCheckDialogArgs {
    pub(super) notice_sender:  ui::SyncNoticeSender,
    pub(super) config: ConnectConfig,
}

impl ConnectCheckDialogArgs {
    pub fn new(notice: &ui::SyncNotice, config: ConnectConfig) -> Self {
        Self {
            notice_sender: notice.sender(),
            config,
        }
    }

    pub fn send_notice(&self) {
        self.notice_sender.send()
    }
}

impl ui::PopupArgs for ConnectCheckDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
