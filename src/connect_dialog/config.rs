
#[derive(Default, Debug, Clone)]
pub struct ConnectConfig {
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub enable_tls: bool,
    pub accept_invalid_tls: bool,
}
