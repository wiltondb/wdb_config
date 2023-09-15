
use super::*;

#[derive(Default)]
pub(super) struct AboutDialogControls {
    layout: AboutDialogLayout,

    pub(super) font_normal: nwg::Font,

    pub(super) icon: nwg::Icon,
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

        nwg::Icon::builder()
            .source_embed(Some(&nwg::EmbedResource::load(None)
                .expect("Error loading embedded resource")))
            .source_embed_id(2)
            .build(&mut self.icon)?;

        nwg::Window::builder()
            .size((320, 120))
            .icon(Some(&self.icon))
            .center(true)
            .title("About")
            .build(&mut self.window)?;

        nwg::Label::builder()
            .text(&format!("Configuration tool for WiltonDB.\r\nVersion {}.", labels::VERSION))
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

        self.layout.build(&self)?;

        Ok(())
    }

    fn update_tab_order(&self) {
        ()
    }
}
