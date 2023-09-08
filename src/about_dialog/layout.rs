
use super::*;

#[derive(Default)]
pub(super) struct AboutDialogLayout {
    root_layout: nwg::FlexboxLayout,
}

impl ui::Layout<AboutDialogControls> for AboutDialogLayout {
    fn build(&self, c: &AboutDialogControls) -> Result<(), nwg::NwgError> {
        nwg::FlexboxLayout::builder()
            .parent(&c.window)
            .flex_direction(ui::FlexDirection::Column)

            .child(&c.label)
            .child_size(ui::size_builder()
                .width_auto()
                .height_pt(50)
                .build())
            .child_flex_grow(1.0)

            .child(&c.close_button)
            .child_size(ui::size_builder()
                .width_button_normal()
                .height_button()
                .build())
            .child_align_self(ui::AlignSelf::FlexEnd)

            .build(&self.root_layout)?;

        Ok(())
    }
}
