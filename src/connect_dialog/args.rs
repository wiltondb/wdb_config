
use super::*;

#[derive(Default)]
pub struct ConnectDialogArgs {
    notice_sender:  ui::SyncNoticeSender,
    pub(super) pg_conn_config: PgConnConfig,
}

impl ConnectDialogArgs {
    pub fn new(notice: &ui::SyncNotice, pg_conn_config: PgConnConfig) -> Self {
        Self {
            notice_sender: notice.sender(),
            pg_conn_config,
        }
    }
}

impl ui::PopupArgs for ConnectDialogArgs {
    fn notify_parent(&self) {
        self.notice_sender.send()
    }
}
