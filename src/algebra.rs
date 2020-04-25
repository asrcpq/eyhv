extern crate derive_more;
use derive_more::{Add, AddAssign, Sub, SubAssign, Mul, Div};

// Point2f is also Vec2f
#[derive(Copy, Clone, Add, AddAssign, Sub, SubAssign, Mul, Div)]
pub struct Point2f {
    pub x: f32,
    pub y: f32,
}

impl Point2f {
    pub fn new() -> Point2f {
        Point2f {
            x: 0.,
            y: 0.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct LineSeg2f {
    pub begin: Point2f,
    pub end: Point2f,
}

#[derive(Clone)]
pub struct Polygen2f {
    pub nodes: Vec<Point2f>,
}
