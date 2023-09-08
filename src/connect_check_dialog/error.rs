
use super::*;

#[derive(Debug)]
pub(super) struct ConnectCheckDialogError {
    message: String
}

impl ConnectCheckDialogError {
    fn new<E: fmt::Display>(e: &E) -> Self {
        Self {
            message: format!("{}", e)
        }
    }
}

impl fmt::Display for ConnectCheckDialogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<postgres::Error> for ConnectCheckDialogError {
    fn from(value: postgres::Error) -> Self {
        Self::new(&value)
    }
}

impl From<native_tls::Error> for ConnectCheckDialogError {
    fn from(value: native_tls::Error) -> Self {
        Self::new(&value)
    }
}
