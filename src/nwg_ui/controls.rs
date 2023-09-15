

pub trait Controls {
     fn build(&mut self) -> Result<(), nwg::NwgError>;

     fn update_tab_order(&self);
}
