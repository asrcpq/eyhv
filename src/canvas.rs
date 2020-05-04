pub struct Canvas {
    pub data: Vec<u8>,
    size: (u32, u32),
}

impl Canvas {
    pub fn new(size: (u32, u32)) -> Canvas {
        Canvas {
            data: vec![0; (size.0 * size.1 * 3) as usize],
            size,
        }
    }

    #[inline]
    pub fn map(&self, x: u32, y: u32) -> usize {
        (y * self.size.0 + x) as usize
    }
}
