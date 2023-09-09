
use super::*;

#[derive(Default)]
pub struct LoadSettingsDialogArgs {
    notice_sender:  ui::SyncNoticeSender,
    pub(super) config: ConnectConfig,
}

impl LoadSettingsDialogArgs {
    pub fn new(notice: &ui::SyncNotice, config: ConnectConfig) -> Self {
        Self {
            notice_sender: notice.sender(),
            config,
        }
    }
}

impl ui::PopupArgs for LoadSettingsDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
