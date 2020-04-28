use std::collections::VecDeque;

use crate::algebra::{Point2f, Mat2x2f};
use crate::graphic_objects::GraphicObjects;

// This struct is static, created by Session::new() only once
pub struct BulletGraphicObjects {
    wedge: GraphicObjects,
}

pub const bullet_graphic_objects: BulletGraphicObjects<'static> = BulletGraphicObjects {
    wedge: GraphicObjects::from_strs(
        vec!["l 1 1 0 1 -3 -5 3 -5 0 5 -3 -5"]
    ),
}

pub trait Bullet {
    fn tick(&mut self, dt: f32);
    fn get_p(&self) -> Point2f;
}

pub struct CommonBullet {
    p: Point2f,
    v: Point2f,
    a: Point2f,
    theta: f32,
    graphic_objects: GraphicObjects,
}

pub struct RotateBullet {
    p: Point2f,
    v: Point2f,
    a: Point2f,
    theta: f32,
    rotate_matrix: Mat2x2f,
    graphic_objects: GraphicObjects,
}
