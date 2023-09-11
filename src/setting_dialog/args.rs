
use super::*;

#[derive(Default)]
pub struct SettingDialogArgs {
    notice_sender:  ui::SyncNoticeSender,
    pub(super) pg_conn_config: PgConnConfig,
    pub(super) setting: SettingRecord,
}

impl SettingDialogArgs {
    pub fn new(notice: &ui::SyncNotice, pg_conn_config: PgConnConfig, setting: SettingRecord) -> Self {
        Self {
            notice_sender: notice.sender(),
            pg_conn_config,
            setting
        }
    }
}

impl ui::PopupArgs for SettingDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
