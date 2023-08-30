
//use nwg::stretch::geometry::Rect;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::Dimension;
use nwg::stretch::style::FlexDirection;
//use nwg::stretch::style::AlignSelf;

use super::*;
use about_dialog::AboutDialog;

//const FIFTY_PC: D = D::Percent(0.5);
const PT_50: Dimension = Dimension::Points(50.0);
//const PADDING: Rect<D> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };
//const MARGIN: Rect<D> = Rect{ start: PT_5, end: PT_5, top: PT_5, bottom: PT_5 };

pub fn build(data: &mut AboutDialog) -> Result<(), nwg::NwgError> {
    let root_layout = Default::default();
    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Row)
        .child(&data.choice_yes)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child(&data.choice_no)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child_flex_grow(1.0)
        .build(&root_layout)?;

    Ok(())
}
