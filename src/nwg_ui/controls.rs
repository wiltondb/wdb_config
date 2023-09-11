

pub trait Controls {
     fn build(&mut self) -> Result<(), nwg::NwgError>;
}
