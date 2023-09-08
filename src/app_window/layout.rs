
use super::*;

#[derive(Default)]
pub(super) struct AppWindowLayout {
    root_layout: nwg::FlexboxLayout,
    main_view_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,
}

impl ui::Layout<AppWindowControls> for AppWindowLayout {
    fn build(&self, c: &AppWindowControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .child(&c.main_view)
            .child_flex_grow(1.0)
            .auto_spacing(None)
            .build_partial(&self.main_view_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .justify_content(ui::JustifyContent::FlexEnd)
            .auto_spacing(None)
            .child(&c.reload_button)
            .child_size(ui::size_builder()
                .width_button_wide()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .bottom_pt(22)
                .build())
            .child(&c.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_margin(ui::margin_builder()
                .start_pt(5)
                .bottom_pt(22)
                .build())
            .build_partial(&self.buttons_layout)?;

        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Column)
            .child_layout(&self.main_view_layout)
            .child_flex_grow(1.0)
            .child_layout(&self.buttons_layout)
            .build(&self.root_layout)?;

        Ok(())
    }
}
