

pub trait Controls {
     fn build(&mut self) -> Result<(), nwg::NwgError>;

    fn window(&self) -> &nwg::Window;

    fn hide_window(&self) {
        self.window().set_visible(false);
    }

    fn shake_window(&self) {
        // workaround for garbled text
        let (wx, wy) = self.window().size();
        self.window().set_size(wx + 1, wy + 1);
        self.window().set_size(wx, wy);
    }
}
