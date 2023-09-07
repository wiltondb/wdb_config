
use crate::*;
use nwg_ui as ui;

#[derive(Default)]
pub(super) struct AboutDialogControls {
    pub(super) font_normal: nwg::Font,

    pub(super) window: nwg::Window,
    pub(super) label: nwg::Label,
    pub(super) close_button: nwg::Button,
}

impl ui::Controls for AboutDialogControls {
    fn build(&mut self) -> Result<(), nwg::NwgError> {
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;

        nwg::Window::builder()
            .size((320, 200))
            .center(true)
            .title("About")
            .build(&mut self.window)?;

        nwg::Label::builder()
            .text("Very long label label label label label label label label \r\n will eventually go here")
            .h_align(nwg::HTextAlign::Center)
            .v_align(nwg::VTextAlign::Top)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.label)?;

        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        Ok(())
    }

    fn window(&self) -> &nwg::Window {
        &self.window
    }
}
