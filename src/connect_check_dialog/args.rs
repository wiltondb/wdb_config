
use super::*;

#[derive(Default)]
pub struct ConnectCheckDialogArgs {
    pub(super) notice_sender:  ui::SyncNoticeSender,
    pub(super) pg_conn_config: PgConnConfig,
}

impl ConnectCheckDialogArgs {
    pub fn new(notice: &ui::SyncNotice, pg_conn_config: PgConnConfig) -> Self {
        Self {
            notice_sender: notice.sender(),
            pg_conn_config,
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
