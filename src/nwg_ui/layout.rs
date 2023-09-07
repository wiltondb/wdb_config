
use super::*;

pub trait Layout<C: Controls> {
    fn build(&self, c: &C) -> Result<(), nwg::NwgError>;
}