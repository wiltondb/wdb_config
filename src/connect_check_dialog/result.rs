
#[derive(Default)]
pub(super) struct ConnectCheckResult {
    pub(super) success: bool,
    pub(super) message: String,
}

impl ConnectCheckResult {
    pub(super) fn success(message: String) -> Self {
        Self {
            success: true,
            message
        }
    }

    pub(super) fn failure(message: String) -> Self {
        Self {
            success: false,
            message
        }
    }
}

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
