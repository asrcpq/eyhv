extern crate derive_more;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops::Mul;

// Point2f is also Vec2f
// 2f means 2d+f32
#[derive(
    Copy, Clone, PartialEq, Debug, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign,
)]
pub struct Point2f {
    // / x \
    // \ y /
    pub x: f32,
    pub y: f32,
}

#[derive(
    Copy, Clone, PartialEq, Debug, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign,
)]
pub struct Mat2x2f {
    // / x1 x2 \
    // \ y1 y2 /
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl Mul<Point2f> for Mat2x2f {
    type Output = Point2f;

    fn mul(self, rhs: Point2f) -> Point2f {
        Point2f {
            x: self.x1 * rhs.x + self.x2 * rhs.y,
            y: self.y1 * rhs.x + self.y2 * rhs.y,
        }
    }
}

impl Mat2x2f {
    pub fn from_theta(theta: f32) -> Mat2x2f {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        Mat2x2f {
            x1: cos_theta,
            x2: -sin_theta,
            y1: sin_theta,
            y2: cos_theta,
        }
    }
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
pub struct Rect2f {
    lu: Point2f,
    rd: Point2f,
}

impl Rect2f {
    pub fn new() -> Rect2f {
        Rect2f {
            lu: Point2f::new(),
            rd: Point2f::new(),
        }
    }

    pub fn from_point2fs(lu: Point2f, rd: Point2f) -> Rect2f {
        Rect2f { lu: lu, rd: rd }
    }

    // check if point2f falls inside the rectangle(not falls on)
    pub fn contains(&self, point2f: Point2f) -> bool {
        point2f.x > self.lu.x
            && point2f.x < self.rd.x
            && point2f.y > self.lu.y
            && point2f.y < self.rd.y
    }

    // find the nearest point in the rectangle to a given point
    pub fn nearest(&self, point2f: Point2f) -> Point2f {
        let mut nearest_point = Point2f::new();
        if point2f.x < self.lu.x {
            nearest_point.x = self.lu.x;
        } else if point2f.x > self.rd.x {
            nearest_point.x = self.rd.x;
        } else {
            nearest_point.x = point2f.x;
        }
        if point2f.y < self.lu.y {
            nearest_point.y = self.lu.y;
        } else if point2f.y > self.rd.y {
            nearest_point.y = self.rd.y;
        } else {
            nearest_point.y = point2f.y;
        }
        nearest_point
    }
}

#[cfg(test)]
mod test {
    use super::{Point2f, Mat2x2f};

    #[test]
    fn test_point2f_derive_more() {
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

    #[test]
    fn test_mat2x2f() {
        let mat2x2f = Mat2x2f::from_theta(std::f32::consts::PI / 2.);
        let eps:f32 = 1e-6;
        // / 0 -1 \
        // \ 1  0 /
        assert!(mat2x2f.x1.abs() < eps);
        assert!((mat2x2f.x2 + 1.).abs() < eps);
        assert!((mat2x2f.y1 - 1.).abs() < eps);
        assert!(mat2x2f.y2.abs() < eps);

        // Rotate
        let point2f = Point2f::from_floats(3.0, 4.0);
        let point2f = mat2x2f * point2f;
        assert!((point2f.x + 4.).abs() < eps);
        assert!((point2f.y - 3.).abs() < eps);
    }
}
