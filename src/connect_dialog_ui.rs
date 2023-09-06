use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use nwg::stretch::style::AlignSelf;
use nwg::stretch::style::JustifyContent;
use nwg::stretch::style::FlexDirection;

use crate::*;
use dialogs::DialogUi;
use dialogs::PopupDialog;
use connect_dialog::ConnectConfig;
use connect_dialog::ConnectDialog;
use nwg::Window;

#[derive(Default)]
pub struct ConnectDialogUi {
    events: events::Events<ConnectDialog>,
    font_normal: nwg::Font,

    window: nwg::Window,
    hostname_label: nwg::Label,
    hostname_input: nwg::TextInput,
    port_label: nwg::Label,
    port_input: nwg::TextInput,
    username_label: nwg::Label,
    username_input: nwg::TextInput,
    password_label: nwg::Label,
    password_input: nwg::TextInput,
    enable_tls_checkbox: nwg::CheckBox,
    accept_invalid_tls_checkbox: nwg::CheckBox,

    test_button: nwg::Button,
    close_button: nwg::Button,

    root_layout: nwg::FlexboxLayout,
    hostname_layout: nwg::FlexboxLayout,
    port_layout: nwg::FlexboxLayout,
    username_layout: nwg::FlexboxLayout,
    password_layout: nwg::FlexboxLayout,
    enable_tls_layout: nwg::FlexboxLayout,
    accept_invalid_tls_layout: nwg::FlexboxLayout,

    spacer_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,

    check_dialog_notice: notice::SyncNotice,
}

impl ConnectDialogUi {
    pub fn check_dialog_notice(&self) -> &notice::SyncNotice {
        &self.check_dialog_notice
    }

    pub fn correct_port_value(&self) {
        let text = self.port_input.text();
        if text.len() == 0 {
            self.port_input.set_text("1");
            return;
        }
        let num = match text.parse::<u128>() {
            Err(_) => {
                self.port_input.set_text("5432");
                return;
            },
            Ok(n) => n
        };
        if num > 65535 {
            self.port_input.set_text("65535");
        }
    }

    pub fn config_from_input(&self) -> ConnectConfig {
        let port = match self.port_input.text().parse::<u16>() {
            Ok(n) => n,
            Err(_) => 5432,
        };
        ConnectConfig {
            hostname: self.hostname_input.text(),
            port,
            username: self.username_input.text(),
            password: self.password_input.text(),
            enable_tls: self.enable_tls_checkbox.check_state() == nwg::CheckBoxState::Checked,
            accept_invalid_tls: self.enable_tls_checkbox.enabled() &&
                self.accept_invalid_tls_checkbox.check_state() == nwg::CheckBoxState::Checked
        }
    }

    pub fn sync_tls_checkboxes_state(&self) {
        let enabled = self.enable_tls_checkbox.check_state() == nwg::CheckBoxState::Checked;
        self.accept_invalid_tls_checkbox.set_enabled(enabled);
    }
}

impl DialogUi for ConnectDialogUi {
    fn window(&self) -> &Window {
        &self.window
    }

    fn build_controls(&mut self) -> Result<(), nwg::NwgError> {
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
        events::builder()
            .control(&self.window)
            .event(nwg::Event::OnWindowClose)
            .handler(ConnectDialog::close)
            .build(&mut self.events)?;

        nwg::Label::builder()
            .text("Hostname:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .parent(&self.window)
            .build(&mut self.hostname_label)?;
        nwg::TextInput::builder()
            .text("localhost")
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
            .text("5432")
            .flags(nwg::TextInputFlags::VISIBLE | nwg::TextInputFlags::NUMBER)
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.port_input)?;
        events::builder()
            .control(&self.port_input)
            .event(nwg::Event::OnTextInput)
            .handler(ConnectDialog::on_port_input_changed)
            .build(&mut self.events)?;
        nwg::Label::builder()
            .text("Username:")
            .font(Some(&self.font_normal))
            .h_align(nwg::HTextAlign::Left)
            .parent(&self.window)
            .build(&mut self.username_label)?;
        nwg::TextInput::builder()
            .text("wilton")
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
            .text("wilton")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.password_input)?;
        nwg::CheckBox::builder()
            .check_state(nwg::CheckBoxState::Checked)
            .text("Enable TLS")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.enable_tls_checkbox)?;
        events::builder()
            .control(&self.enable_tls_checkbox)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectDialog::on_enable_tls_checkbox_changed)
            .build(&mut self.events)?;
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
        events::builder()
            .control(&self.test_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectDialog::open_check_dialog)
            .build(&mut self.events)?;

        nwg::Button::builder()
            .text("Close")
            .font(Some(&self.font_normal))
            .parent(&self.window)
            .build(&mut self.close_button)?;
        events::builder()
            .control(&self.close_button)
            .event(nwg::Event::OnButtonClick)
            .handler(ConnectDialog::close)
            .build(&mut self.events)?;

        notice::builder()
            .parent(&self.window)
            .build(&mut self.check_dialog_notice)?;
        events::builder()
            .control(&self.check_dialog_notice.notice)
            .event(nwg::Event::OnNotice)
            .handler(ConnectDialog::await_check_dialog)
            .build(&mut self.events)?;

        Ok(())
    }

    fn build_layout(&mut self) -> Result<(), nwg::NwgError> {

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .auto_spacing(None)
            .child(&self.hostname_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&self.hostname_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.hostname_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .auto_spacing(None)
            .child(&self.port_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&self.port_input)
            .child_size(ui::size_builder()
                .width_number_input_normal()
                .height_input_form_row()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .build_partial(&self.port_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .auto_spacing(None)
            .child(&self.username_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&self.username_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.username_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .auto_spacing(None)
            .child(&self.password_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&self.password_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.password_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .auto_spacing(None)
            .child(&self.enable_tls_checkbox)
            .child_size(ui::size_builder()
                .width_auto()
                .height_input_form_row()
                .build())
            .child_flex_grow(1.0)
            .child_margin(ui::margin_builder()
                .start_no_label_normal()
                .build())
            .build_partial(&self.enable_tls_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .auto_spacing(None)
            .child(&self.accept_invalid_tls_checkbox)
            .child_size(ui::size_builder()
                .width_auto()
                .height_input_form_row()
                .build())
            .child_flex_grow(1.0)
            .child_margin(ui::margin_builder()
                .start_no_label_normal()
                .build())
            .build_partial(&self.accept_invalid_tls_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .auto_spacing(None)
            .build_partial(&self.spacer_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Row)
            .justify_content(JustifyContent::FlexEnd)
            .auto_spacing(None)
            .child(&self.test_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child(&self.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .build_partial(&self.buttons_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&self.window)
            .flex_direction(FlexDirection::Column)
            .child_layout(&self.hostname_layout)
            .child_layout(&self.port_layout)
            .child_layout(&self.username_layout)
            .child_layout(&self.password_layout)
            .child_layout(&self.enable_tls_layout)
            .child_layout(&self.accept_invalid_tls_layout)
            .child_layout(&self.spacer_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.buttons_layout)
            .build(&mut self.root_layout)?;

        Ok(())
    }

}

pub struct ConnectDialogNwg {
    inner: Rc<ConnectDialog>,
    default_handler: RefCell<Option<nwg::EventHandler>>
}

impl nwg::NativeUi<ConnectDialogNwg> for ConnectDialog {
    fn build_ui(mut data: ConnectDialog) -> Result<ConnectDialogNwg, nwg::NwgError> {
        data.build_popup_ui()?;

        let wrapper = ConnectDialogNwg {
            inner:  Rc::new(data),
            default_handler: Default::default(),
        };

        let data_ref = Rc::downgrade(&wrapper.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(evt_data) = data_ref.upgrade() {
                for eh in evt_data.ui().events.iter() {
                    if handle == eh.control_handle && evt == eh.event {
                        (eh.handler)(&evt_data);
                        break;
                    }
                }
            }
        };

        *wrapper.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(&wrapper.ui().window.handle, handle_events));

        return Ok(wrapper);
    }
}

impl Drop for ConnectDialogNwg {
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for ConnectDialogNwg {
    type Target = ConnectDialog;

    fn deref(&self) -> &ConnectDialog {
        &self.inner
    }
}
