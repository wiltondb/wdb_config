
use super::*;

#[derive(Default)]
pub(super) struct ConnectDialogControls {
    layout: ConnectDialogLayout,

    pub(super) font_normal: nwg::Font,

    pub(super) icon: nwg::Icon,
    pub(super) window: nwg::Window,

    pub(super) hostname_label: nwg::Label,
    pub(super) hostname_input: nwg::TextInput,
    pub(super) port_label: nwg::Label,
    pub(super) port_input: nwg::TextInput,
    pub(super) username_label: nwg::Label,
    pub(super) username_input: nwg::TextInput,
    pub(super) password_label: nwg::Label,
    pub(super) password_input: nwg::TextInput,
    pub(super) enable_tls_checkbox: nwg::CheckBox,
    pub(super) accept_invalid_tls_checkbox: nwg::CheckBox,

    pub(super) test_button: nwg::Button,
    pub(super) load_button: nwg::Button,
    pub(super) cancel_button: nwg::Button,

    pub(super) check_notice: ui::SyncNotice,
}

impl ui::Controls for ConnectDialogControls {

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
            .size((480, 240))
            .icon(Some(&self.icon))
            .center(true)
            .title("DB Connection")
            .build(&mut self.window)?;

        nwg::Label::builder()
            .text("Hostname:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .parent(&self.window)
            .build(&mut self.hostname_label)?;
        nwg::TextInput::builder()
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.hostname_input)?;
        nwg::Label::builder()
            .text("Port:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .parent(&self.window)
            .build(&mut self.port_label)?;
        nwg::TextInput::builder()
            .flags(nwg::TextInputFlags::VISIBLE | nwg::TextInputFlags::NUMBER)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.port_input)?;
        nwg::Label::builder()
            .text("Username:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .parent(&self.window)
            .build(&mut self.username_label)?;
        nwg::TextInput::builder()
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.username_input)?;
        nwg::Label::builder()
            .text("Password:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .parent(&self.window)
            .build(&mut self.password_label)?;
        nwg::TextInput::builder()
            .password(Some('*'))
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.password_input)?;
        nwg::CheckBox::builder()
            .check_state(nwg::CheckBoxState::Checked)
            .text("Enable TLS")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.enable_tls_checkbox)?;
        nwg::CheckBox::builder()
            .check_state(nwg::CheckBoxState::Checked)
            .text("Accept invalid TLS certificates/hosts")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.accept_invalid_tls_checkbox)?;

        nwg::Button::builder()
            .text("Test connection")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.test_button)?;

        nwg::Button::builder()
            .text("Load settings")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.load_button)?;

        nwg::Button::builder()
            .text("Cancel")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.cancel_button)?;

        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.check_notice)?;

        self.layout.build(&self)?;

        Ok(())
    }
}
