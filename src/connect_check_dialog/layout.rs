
use super::*;

#[derive(Default)]
pub(super) struct ConnectCheckDialogLayout {
    root_layout: nwg::FlexboxLayout,
    buttons_layout: nwg::FlexboxLayout,
}

impl ui::Layout<ConnectCheckDialogControls> for ConnectCheckDialogLayout {
    fn build(&self, c: &ConnectCheckDialogControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Row)
            .justify_content(ui::JustifyContent::FlexEnd)
            .auto_spacing(None)

            .child(&c.copy_clipboard_button)
            .child_size(ui::size_builder()
                .width_button_xwide()
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

            .child(&c.progress_bar)
            .child_size(ui::size_builder()
                .height_pt(30)
                .width_auto()
                .build())
            .child_align_self(ui::AlignSelf::Stretch)

            .child(&c.label)
            .child_size(ui::size_builder()
                .height_pt(10)
                .width_auto()
                .build())
            .child_align_self(ui::AlignSelf::Stretch)

            .child(&c.details_box)
            .child_size(ui::size_builder()
                .height_auto()
                .width_auto()
                .build())
            .child_align_self(ui::AlignSelf::Stretch)
            .child_flex_grow(1.0)

            .child_layout(&self.buttons_layout)
            .child_align_self(ui::AlignSelf::Stretch)

            .build(&self.root_layout)?;

        Ok(())
    }
}
