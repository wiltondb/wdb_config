
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
