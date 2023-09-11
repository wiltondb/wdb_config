
use std::fmt;

#[derive(Debug)]
pub struct PgConnError {
    message: String
}

impl PgConnError {
    fn new<E: fmt::Display>(e: &E) -> Self {
        Self {
            message: format!("{}", e)
        }
    }
}

impl fmt::Display for PgConnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<postgres::Error> for PgConnError {
    fn from(value: postgres::Error) -> Self {
        Self::new(&value)
    }
}

impl From<native_tls::Error> for PgConnError {
    fn from(value: native_tls::Error) -> Self {
        Self::new(&value)
    }
}
