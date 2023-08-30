
//use nwg::stretch::geometry::Rect;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::Dimension;
use nwg::stretch::style::FlexDirection;
//use nwg::stretch::style::AlignSelf;

use super::*;
use app_window::AppWindow;

//const FIFTY_PC: D = D::Percent(0.5);
const PT_50: Dimension = Dimension::Points(50.0);
//const PADDING: Rect<D> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };
//const MARGIN: Rect<D> = Rect{ start: PT_5, end: PT_5, top: PT_5, bottom: PT_5 };


pub fn build(data: &mut AppWindow) -> Result<(), nwg::NwgError> {
    let root_layout = Default::default();
    let row1_layout = Default::default();
    let row2_layout = Default::default();
    let row3_layout = Default::default();

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Row)
        .child(&data.button1)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child(&data.button2)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child_flex_grow(1.0)
        .build_partial(&row1_layout)?;

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Row)
        .child(&data.data_view)
        .child_flex_grow(1.0)
        .build_partial(&row2_layout)?;

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Row)
        .child(&data.button5)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child(&data.button6)
        .child_size(Size { width: PT_50, height: PT_50 })
        .child_flex_grow(1.0)
        .build_partial(&row3_layout)?;

    nwg::FlexboxLayout::builder()
        .parent(&data.window)
        .flex_direction(FlexDirection::Column)
        .child_layout(&row1_layout)
        .child_layout(&row2_layout)
        .child_flex_grow(1.0)
        .child_layout(&row3_layout)
        .build(&root_layout)?;

    Ok(())
}