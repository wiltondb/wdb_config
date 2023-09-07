
use nwg::stretch::geometry::Rect;
use nwg::stretch::style::Dimension;

#[derive(Default)]
pub struct MarginBuilder {
    start: u32,
    end: u32,
    top: u32,
    bottom: u32,
}

#[allow(dead_code)]
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
