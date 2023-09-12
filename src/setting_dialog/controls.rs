
use super::*;

#[derive(Default)]
pub(super) struct SettingDialogControls {
    layout: SettingDialogLayout,

    pub(super) font_normal: nwg::Font,

    pub(super) window: nwg::Window,

    pub(super) name_label: nwg::Label,
    pub(super) name_input: nwg::TextInput,
    pub(super) current_value_label: nwg::Label,
    pub(super) current_value_input: nwg::TextInput,
    pub(super) new_value_label: nwg::Label,
    pub(super) new_value_input: nwg::TextInput,
    pub(super) description_label: nwg::Label,

    pub(super) change_button: nwg::Button,
    pub(super) close_button: nwg::Button,

    pub(super) change_notice: ui::SyncNotice,
}

impl ui::Controls for SettingDialogControls {
    fn build(&mut self) -> Result<(), nwg::NwgError> {
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;

        nwg::Window::builder()
            .size((480, 200))
            .center(true)
            .title("Change setting")
            .build(&mut self.window)?;

        nwg::Label::builder()
            .text("Setting name:")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.name_label)?;
        nwg::TextInput::builder()
            .readonly(true)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.name_input)?;

        nwg::Label::builder()
            .text("Current value:")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.current_value_label)?;
        nwg::TextInput::builder()
            .readonly(true)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.current_value_input)?;

        nwg::Label::builder()
            .text("New value:")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.new_value_label)?;
        nwg::TextInput::builder()
            .flags(nwg::TextInputFlags::VISIBLE)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.new_value_input)?;

        nwg::Label::builder()
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.description_label)?;

        nwg::Button::builder()
            .text("Apply change")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.change_button)?;
        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.change_notice)?;

        self.layout.build(&self)?;

        Ok(())
    }
}
