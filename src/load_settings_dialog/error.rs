
use super::*;

#[derive(Debug)]
pub(super) struct LoadSettingsDialogError {
    message: String
}

impl LoadSettingsDialogError {
    fn new<E: fmt::Display>(e: &E) -> Self {
        Self {
            message: format!("{}", e)
        }
    }
}

impl fmt::Display for LoadSettingsDialogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<postgres::Error> for LoadSettingsDialogError {
    fn from(value: postgres::Error) -> Self {
        Self::new(&value)
    }
}

impl From<native_tls::Error> for LoadSettingsDialogError {
    fn from(value: native_tls::Error) -> Self {
        Self::new(&value)
    }
}
