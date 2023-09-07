

pub struct ConnectCheckDialogResult {
    pub value: Result<String, postgres::Error>
}

impl Default for ConnectCheckDialogResult {
    fn default() -> Self {
        Self {
            value: Ok(String::new())
        }
    }
}

