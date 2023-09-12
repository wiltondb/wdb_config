
use super::*;

#[derive(Default)]
pub struct SettingChangeDialogArgs {
    pub(super) notice_sender:  ui::SyncNoticeSender,
    pub(super) pg_conn_config: PgConnConfig,
    pub(super) name: String,
    pub(super) value: String,
}

impl SettingChangeDialogArgs {
    pub fn new(notice: &ui::SyncNotice, pg_conn_config: PgConnConfig, name: String, value: String) -> Self {
        Self {
            notice_sender: notice.sender(),
            pg_conn_config,
            name,
            value,
        }
    }

    pub fn send_notice(&self) {
        self.notice_sender.send()
    }
}

impl ui::PopupArgs for SettingChangeDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
