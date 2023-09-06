
use nwg::stretch::geometry::Rect;
use nwg::stretch::geometry::Size;
use nwg::stretch::style::Dimension;

//pub const FONT_SIZE_NORMAL: u32 = 15;

//pub const SIZE_BUTTON_NORMAL: Size<Dimension> = width_height(Dimension::Points(70.0), Dimension::Points(25.0));

//pub const PT_50: Dimension = Dimension::Points(50.0);
//pub const PC_100: Dimension = Dimension::Percent(100.0);
//const PADDING: Rect<Dimension> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };
//const MARGIN: Rect<Dimension> = Rect{ start: PT_5, end: PT_5, top: PT_5, bottom: PT_5 };

#[derive(Default)]
pub struct SizeBuilder {
    width: Dimension,
    height: Dimension,
}

impl SizeBuilder {
    pub fn width(mut self, width: Dimension) -> Self {
        self.width = width;
        self
    }

    pub fn width_percent(mut self, width: u32) -> Self {
        self.width = Dimension::Percent(width as f32);
        self
    }

    pub fn width_points(mut self, width: u32) -> Self {
        self.width = Dimension::Points(width as f32);
        self
    }

    pub fn width_button_normal(mut self) -> Self {
        self.width = Dimension::Points(70 as f32);
        self
    }

    pub fn width_button_wide(mut self) -> Self {
        self.width = Dimension::Points(100 as f32);
        self
    }

    pub fn width_button_xwide(mut self) -> Self {
        self.width = Dimension::Points(130 as f32);
        self
    }
     pub fn width_input_default(mut self) -> Self {
         self.width = Dimension::Points(100 as f32);
         self
     }

    pub fn width_label_normal(mut self) -> Self {
        self.width = Dimension::Points(100 as f32);
        self
    }

    pub fn width_number_input_normal(mut self) -> Self {
        self.width = Dimension::Points(80 as f32);
        self
    }

    pub fn width_checkbox_normal(mut self) -> Self {
        self.width = Dimension::Points(30 as f32);
        self
    }

    pub fn width_auto(mut self) -> Self {
        self.width = Dimension::Auto;
        self
    }

    pub fn height(mut self, height: Dimension) -> Self {
        self.height = height;
        self
    }

    pub fn height_percent(mut self, height: u32) -> Self {
        self.height = Dimension::Percent(height as f32);
        self
    }

    pub fn height_pt(mut self, height: u32) -> Self {
        self.height = Dimension::Points(height as f32);
        self
    }

    pub fn height_button(mut self) -> Self {
        self.height = Dimension::Points(25 as f32);
        self
    }

    pub fn height_input_form_row(mut self) -> Self {
        self.height = Dimension::Points(20 as f32);
        self
    }

    pub fn height_auto(mut self) -> Self {
        self.height = Dimension::Auto;
        self
    }

    pub fn build(self) -> Size<Dimension> {
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

    pub fn small(mut self) -> Self {
        self.size = 14;
        self
    }

    pub fn build(self) -> u32 {
        self.size
    }
}

pub fn font_size_builder() -> FontSizeBuilder {
    Default::default()
}

#[derive(Default)]
pub struct MarginBuilder {
    start: u32,
    end: u32,
    top: u32,
    bottom: u32,
}

impl MarginBuilder {
    pub fn start_pt(mut self, start: u32) -> Self {
        self.start = start;
        self
    }

    pub fn start_default(mut self) -> Self {
        self.start = 5;
        self
    }

    pub fn start_no_label_normal(mut self) -> Self {
        self.start = 105;
        self
    }

    pub fn end_pt(mut self, end: u32) -> Self {
        self.end = end;
        self
    }

    pub fn end_default(mut self) -> Self {
        self.end = 5;
        self
    }

    pub fn top_pt(mut self, top: u32) -> Self {
        self.top = top;
        self
    }

    pub fn top_default(mut self) -> Self {
        self.top = 5;
        self
    }

    pub fn bottom_pt(mut self, bottom: u32) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn bottom_default(mut self) -> Self {
        self.bottom = 5;
        self
    }

    pub fn build(self) -> Rect<Dimension> {
        Rect {
            start: Dimension::Points(self.start as f32),
            end: Dimension::Points(self.end as f32),
            top: Dimension::Points(self.top as f32),
            bottom: Dimension::Points(self.bottom as f32),
        }
    }
}

pub fn margin_builder() -> MarginBuilder {
    Default::default()
}
