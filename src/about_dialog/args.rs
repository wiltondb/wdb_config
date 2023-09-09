
use super::*;

#[derive(Default)]
pub struct AboutDialogArgs {
    notice_sender: ui::SyncNoticeSender
}

impl AboutDialogArgs {
    pub fn new(notice: &ui::SyncNotice) -> Self {
        Self {
            notice_sender: notice.sender()
        }
    }
}

impl ui::PopupArgs for AboutDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
