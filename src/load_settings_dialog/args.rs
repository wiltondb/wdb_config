
use super::*;

#[derive(Default)]
pub struct LoadSettingsDialogArgs {
    notice_sender:  ui::SyncNoticeSender,
    pub(super) pg_conn_config: PgConnConfig,
}

impl LoadSettingsDialogArgs {
    pub fn new(notice: &ui::SyncNotice, pg_conn_config: PgConnConfig) -> Self {
        Self {
            notice_sender: notice.sender(),
            pg_conn_config,
        }
    }
}

impl ui::PopupArgs for LoadSettingsDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
