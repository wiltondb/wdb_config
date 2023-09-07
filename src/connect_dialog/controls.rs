
use crate::*;
use nwg_ui as ui;

#[derive(Default)]
pub(super) struct ConnectDialogControls {
    pub(super) font_normal: nwg::Font,

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
    pub(super) close_button: nwg::Button,

    pub(super) check_notice: ui::SyncNotice,
}

impl ui::Controls for ConnectDialogControls {

    fn build(&mut self) -> Result<(), nwg::NwgError> {
        nwg::Font::builder()
            .size(ui::font_size_builder()
                .normal()
                .build())
            .build(&mut self.font_normal)?;

        nwg::Window::builder()
            .size((480, 240))
            .center(true)
            .title("Connect")
            .build(&mut self.window)?;

        nwg::Label::builder()
            .text("Hostname:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .parent(&self.window)
            .build(&mut self.hostname_label)?;
        nwg::TextInput::builder()
            .text("&args.config.hostname")
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
            .text("&args.config.port.to_string()")
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
            .text("&args.config.username")
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
            .text("&args.config.password")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.password_input)?;
        // todo
        nwg::CheckBox::builder()
            .check_state(nwg::CheckBoxState::Checked)
            .text("Enable TLS")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.enable_tls_checkbox)?;
        // todo
        nwg::CheckBox::builder()
            .check_state(nwg::CheckBoxState::Checked)
            .text("Accept invalid TLS certificates/hosts")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.accept_invalid_tls_checkbox)?;

        nwg::Button::builder()
            .text("Test")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.test_button)?;

        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;

        ui::notice_builder()
            .parent(&self.window)
            .build(&mut self.check_notice)?;

        Ok(())
    }

    fn window(&self) -> &nwg::Window {
        &self.window
    }
}
