extern crate derive_more;
use derive_more::{Add, AddAssign, Sub, SubAssign};

// Point2f is also Vec2f
#[derive(Copy, Clone, Add, AddAssign, Sub, SubAssign)]
pub struct Point2f {
    x: f32,
    y: f32,
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
