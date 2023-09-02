
use nwg::stretch::geometry::Size;
use nwg::stretch::style::{ Dimension as D };

pub const FONT_SIZE_NORMAL: u32 = 15;

pub const SIZE_BUTTON_NORMAL: Size<D> = width_height(D::Points(70.0), D::Points(25.0));

pub const PT_50: D = D::Points(50.0);
//pub const PC_100: D = D::Percent(100.0);
//const PADDING: Rect<D> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };
//const MARGIN: Rect<D> = Rect{ start: PT_5, end: PT_5, top: PT_5, bottom: PT_5 };

pub const fn width_height(width: D, height: D) -> Size<D> {
    Size { width, height}
}

pub const fn height(height: D) -> Size<D> {
    Size { width: D::Auto, height}
}

/*
pub trait UiControls<W: nwg::NativeUi<T>> {
    fn build(data: &mut W) -> Result<(), nwg::NwgError>;
}
 */