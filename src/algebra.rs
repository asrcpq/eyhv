extern crate derive_more;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// Point2f is also Vec2f
#[derive(
    Copy, Clone, PartialEq, Debug, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign,
)]
pub struct Point2f {
    pub x: f32,
    pub y: f32,
}

impl Point2f {
    pub fn new() -> Point2f {
        Point2f { x: 0., y: 0. }
    }
    pub fn from_floats(x: f32, y: f32) -> Point2f {
        Point2f { x: x, y: y }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LineSeg2f {
    pub begin: Point2f,
    pub end: Point2f,
}

#[derive(Clone, Debug)]
pub struct Polygen2f {
    pub vertices: Vec<Point2f>,
}

impl Polygen2f {
    pub fn from_vec(vertices: Vec<Point2f>) -> Polygen2f {
        Polygen2f { vertices: vertices }
    }
    pub fn from_floats(floats: Vec<f32>) -> Polygen2f {
        let mut vertices: Vec<Point2f> = Vec::new();
        let mut iter = floats.iter();
        while match iter.next() {
            Some(v1) => match iter.next() {
                Some(v2) => {
                    vertices.push(Point2f::from_floats(*v1, *v2));
                    true
                }
                None => panic!("odd parse"),
            },
            None => false,
        } {}
        Polygen2f::from_vec(vertices)
    }
}

#[cfg(test)]
mod test {
    use super::Point2f;

    #[test]
    fn test_derive_more() {
        let mut point2f = Point2f::from_floats(1.0, 1.0);
        // PartialEq
        assert_eq!(point2f, Point2f::from_floats(1.0, 1.0));

        // AddAssign
        point2f += Point2f::from_floats(1.0, 1.0);
        assert_eq!(point2f, Point2f::from_floats(2.0, 2.0));

        // MulAssign
        point2f *= 2.;
        assert_eq!(point2f, Point2f::from_floats(4.0, 4.0));
    }
}
