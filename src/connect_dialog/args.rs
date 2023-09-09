
use super::*;

#[derive(Default)]
pub struct ConnectDialogArgs {
    notice_sender:  ui::SyncNoticeSender,
    pub(super) config: ConnectConfig,
}

impl ConnectDialogArgs {
    pub fn new(notice: &ui::SyncNotice, config: ConnectConfig) -> Self {
        Self {
            notice_sender: notice.sender(),
            config,
        }
    }
}

impl ui::PopupArgs for ConnectDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
