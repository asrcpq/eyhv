pub struct Canvas {
    pub data: Vec<u8>,
    size: (i32, i32),
}

impl Canvas {
    pub fn new(size: (i32, i32)) -> Canvas {
        Canvas {
            data: vec![0; (size.0 * size.1 * 3) as usize],
            size,
        }
    }

    pub fn flush(&mut self) {
        self.data = vec![0; (self.size.0 * self.size.1 * 3) as usize];
    }

    #[inline]
    pub fn putpixel(&mut self, x: i32, y: i32, color: &[f32; 4]) {
        if x < 0 || y < 0 || x >= self.size.0 || y >= self.size.1 {
            return;
        }
        for i in 0usize..3usize {
            let pos = ((y * self.size.0 + x) * 3) as usize + i;
            self.data[pos] =
                (self.data[pos] as f32 * (1. - color[3]) + color[i] * 255. * color[3]) as u8;
        }
    }
}
