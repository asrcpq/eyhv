use crate::algebra::Point2f;

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
        ((y * self.size.0 + x) * 3) as usize
    }

    // this is a temporary test function
    #[inline]
    pub fn map_point2f(&self, point2f: Point2f) -> usize {
        (point2f.y as u32 * self.size.0 + point2f.x as u32) as usize
    }
}
