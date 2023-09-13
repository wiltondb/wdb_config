
use super::*;

#[derive(Default, Clone)]
pub struct ConnectDialogResult {
    pub pg_conn_config: PgConnConfig,
    pub load_settings_requested: bool,
}

impl ConnectDialogResult {
    pub fn new(pg_conn_config: PgConnConfig) -> Self {
        Self {
            pg_conn_config,
            load_settings_requested: true
        }
    }

    pub fn cancelled() -> Self {
        Self {
            pg_conn_config: Default::default(),
            load_settings_requested: false
        }
    }
}
