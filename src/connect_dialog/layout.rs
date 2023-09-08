
use super::*;

#[derive(Default)]
pub(super) struct ConnectDialogLayout {
    root_layout: nwg::FlexboxLayout,
    hostname_layout: nwg::FlexboxLayout,
    port_layout: nwg::FlexboxLayout,
    username_layout: nwg::FlexboxLayout,
    password_layout: nwg::FlexboxLayout,
    enable_tls_layout: nwg::FlexboxLayout,
    accept_invalid_tls_layout: nwg::FlexboxLayout,
    spacer_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,
}

impl ui::Layout<ConnectDialogControls> for ConnectDialogLayout {
    fn build(&self, c: &ConnectDialogControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.hostname_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&c.hostname_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.hostname_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.port_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&c.port_input)
            .child_size(ui::size_builder()
                .width_number_input_normal()
                .height_input_form_row()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .build_partial(&self.port_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.username_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&c.username_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.username_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.password_label)
            .child_size(ui::size_builder()
                .width_label_normal()
                .height_input_form_row()
                .build())
            .child(&c.password_input)
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .child_flex_grow(1.0)
            .build_partial(&self.password_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.enable_tls_checkbox)
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
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .child(&c.accept_invalid_tls_checkbox)
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
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .auto_spacing(None)
            .build_partial(&self.spacer_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .justify_content(ui::JustifyContent::FlexEnd)
            .auto_spacing(None)
            .child(&c.test_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child(&c.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .build())
            .build_partial(&self.buttons_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Column)
            .child_layout(&self.hostname_layout)
            .child_layout(&self.port_layout)
            .child_layout(&self.username_layout)
            .child_layout(&self.password_layout)
            .child_layout(&self.enable_tls_layout)
            .child_layout(&self.accept_invalid_tls_layout)
            .child_layout(&self.spacer_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.buttons_layout)
            .build(&self.root_layout)?;

        Ok(())
    }
}
