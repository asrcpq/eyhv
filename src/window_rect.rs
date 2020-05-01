use crate::algebra::{Point2f, Rect2f};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WINDOW_RECT: Rect2f = Rect2f::from_floats(0., 0., 500., 700.);
    pub static ref WINDOW_SIZE: Point2f = WINDOW_RECT.get_size();
}
