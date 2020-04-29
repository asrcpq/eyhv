use crate::algebra::Rect2f;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WINDOW_RECT: Rect2f = Rect2f::from_floats(0., 0., 500., 700.);
}
