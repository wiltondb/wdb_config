
use std::time::Duration;

use native_tls::TlsConnector;
use postgres::Client;
use postgres::Config;
use postgres::NoTls;
use postgres_native_tls::MakeTlsConnector;

use super::*;

#[derive(Default, Debug, Clone)]
pub struct PgConnConfig {
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub enable_tls: bool,
    pub accept_invalid_tls: bool,
}

impl PgConnConfig {
    pub fn open_connection(&self) -> Result<Client, PgConnError> {
        let conf = Config::new()
            .host(&self.hostname)
            .port(self.port)
            .user(&self.username)
            .password(&self.password)
            .connect_timeout(Duration::from_secs(10))
            .clone();

        let res = if self.enable_tls {
            let connector = TlsConnector::builder()
                .danger_accept_invalid_certs(self.accept_invalid_tls)
                .danger_accept_invalid_hostnames(self.accept_invalid_tls)
                .build()?;
            let tls = MakeTlsConnector::new(connector);
            conf.connect(tls)?
        } else {
            conf.connect(NoTls)?
        };

        Ok(res)
    }
}
