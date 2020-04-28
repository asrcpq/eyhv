use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::graphic_objects::GraphicObjects;

// This struct is static, created by Session::new() only once
pub struct BulletGraphicObjects {
    wedge: GraphicObjects,
}

impl BulletGraphicObjects {
    pub fn new() -> BulletGraphicObjects {
        // downward(y+) is forward
        wedge: GraphicObjects::from_strs(
            vec!["l 1 1 0 1 -3 -5 3 -5 0 5 -3 -5"]
        ),
    }
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
    omega: f32,
    graphic_objects: GraphicObjects,
}

pub struct RotateBullet {
    p: Point2f,
    v: Point2f,
    a: Point2f,
    theta: f32,
    graphic_objects: GraphicObjects,
}

impl Bullet {
    pub fn new(p: Point2f, v: Point2f, a: Point2f, theta: f32, omega: f32, graphic_objects: GraphicObjects) -> Bullet {
        Bullet {
            p: p,
            v: v,
            a: a,
            theta: theta,
            omega: omega,
            graphic_objects: graphic_objects,
        }
    }
}
