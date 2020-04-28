use std::collections::VecDeque;

use crate::algebra::{Point2f, Mat2x2f};
use crate::graphic_objects::GraphicObjects;
use crate::moving_object::MovingObject;

// This struct is static, created by Session::new() only once
pub struct BulletGraphicObjects {
    wedge: GraphicObjects,
}

pub const bullet_graphic_objects: BulletGraphicObjects<'static> = BulletGraphicObjects {
    wedge: GraphicObjects::from_strs(
        vec!["l 1 1 0 1 -3 -5 3 -5 0 5 -3 -5"]
    ),
    rectangle: GraphicObjects::from_strs(
        vec!["l 1 1 1 0.5 -1 -5 1 -5 1 5 1 -5 -1 -5"]
    ),

}

pub trait Bullet: MovingObject {
    fn tick(&mut self, dt: f32);
}

pub struct SimpleBullet {
    p: Point2f,
    v: Point2f,
    a: Point2f,
    graphic_objects: GraphicObjects,
}

impl SimpleBullet {
    pub fn new(p: Point2f, v: Point2f, a: Point2f, graphic_objects: GraphicObjects) -> SimpleBullet {
        SimpleBullet {
            p: p,
            v: v,
            a: a,
            graphic_objects: graphic_objects,
        }
    }
}

impl MovingObject for SimpleBullet {
    fn get_p(&self) -> Point2f {
        return p,
    }
    fn moving_object_graphics_iter(&self) -> MovingObjectGraphicsIter {
        MovingObjectGraphicsIter::new(self.p, &self.graphic_objects)
    }
}

impl Bullet for SimpleBullet {
    fn tick(&mut self, dt: f32) {
        self.p += v * dt,
        self.v += a * dt,
    }
}

pub struct RotateBullet {
    p: Point2f,
    v: Point2f,
    a: Point2f,
    // theta is rotate angle, not direction pointing!
    theta: f32,
    rotate_matrix: Mat2x2f,
    graphic_objects: GraphicObjects,
}
