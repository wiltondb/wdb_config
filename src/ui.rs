
use nwg::stretch::geometry::Size;
use nwg::stretch::style::{ Dimension as D };

//pub const FONT_SIZE_NORMAL: u32 = 15;

//pub const SIZE_BUTTON_NORMAL: Size<D> = width_height(D::Points(70.0), D::Points(25.0));

//pub const PT_50: D = D::Points(50.0);
//pub const PC_100: D = D::Percent(100.0);
//const PADDING: Rect<D> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };
//const MARGIN: Rect<D> = Rect{ start: PT_5, end: PT_5, top: PT_5, bottom: PT_5 };

#[derive(Default)]
pub struct SizeBuilder {
    width: D,
    height: D,
}

impl SizeBuilder {
    pub fn width(mut self, width: D) -> Self {
        self.width = width;
        self
    }

    pub fn width_percent(mut self, width: u32) -> Self {
        self.width = D::Percent(width as f32);
        self
    }

    pub fn width_points(mut self, width: u32) -> Self {
        self.width = D::Points(width as f32);
        self
    }

    pub fn width_button_normal(mut self) -> Self {
        self.width = D::Points(70 as f32);
        self
    }

    pub fn width_auto(mut self) -> Self {
        self.width = D::Auto;
        self
    }

    pub fn height(mut self, height: D) -> Self {
        self.height = height;
        self
    }

    pub fn height_percent(mut self, height: u32) -> Self {
        self.height = D::Percent(height as f32);
        self
    }

    pub fn height_points(mut self, height: u32) -> Self {
        self.height = D::Points(height as f32);
        self
    }

    pub fn height_button(mut self) -> Self {
        self.height = D::Points(25 as f32);
        self
    }

    pub fn height_auto(mut self) -> Self {
        self.height = D::Auto;
        self
    }

    pub fn build(self) -> Size<D> {
       Size {
           width: self.width,
           height: self.height
       }
    }
}

pub fn size_builder() -> SizeBuilder {
    Default::default()
}

#[derive(Default)]
pub struct FontSizeBuilder {
    size: u32
}

impl FontSizeBuilder {
    pub fn normal(mut self) -> Self {
        self.size = 15;
        self
    }

    pub fn build(self) -> u32 {
        self.size
    }
}

pub fn font_size_builder() -> FontSizeBuilder {
    Default::default()
}

/*
pub trait UiControls<W: nwg::NativeUi<T>> {
    fn build(data: &mut W) -> Result<(), nwg::NwgError>;
}
 */