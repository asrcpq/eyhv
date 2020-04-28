use lazy_static::lazy_static;

use std::collections::VecDeque;

use crate::algebra::{Point2f, Mat2x2f};
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};

// This struct is static, created by Session::new() only once
pub struct BulletGraphicObjects {
    pub wedge: GraphicObjects,
    pub rectangle: GraphicObjects,
}


lazy_static! {
    pub static ref BULLET_GRAPHIC_OBJECTS: BulletGraphicObjects = {
        BulletGraphicObjects {
            wedge: GraphicObjects::from_strs(
                vec!["l 1 1 0 1 -3 -5 3 -5 0 5 -3 -5"]
            ),
            rectangle: GraphicObjects::from_strs(
                vec!["l 1 1 1 0.5 -1 -5 1 -5 1 5 1 -5 -1 -5"]
            ),
        }
    };
}

pub enum Bullet {
    SimpleBullet(SimpleBullet),
    RotateBullet(RotateBullet),
}

impl Bullet {
    pub fn tick(&mut self, dt: f32) {
        match self {
            Bullet::SimpleBullet(simple_bullet) => {
                simple_bullet.tick(dt);
            },
            Bullet::RotateBullet(rotate_bullet) => {
                unimplemented!();
            },
        }
    }

    fn get_p(&self) -> Point2f {
        match self {
            Bullet::SimpleBullet(simple_bullet) => {
                simple_bullet.get_p()
            },
            Bullet::RotateBullet(rotate_bullet) => {
                unimplemented!()
            },
        }
    }

    fn moving_object_graphics_iter(&self) -> GraphicObjectsIntoIter {
        match self {
            Bullet::SimpleBullet(simple_bullet) => {
                GraphicObjectsIntoIter::new(&self.graphic_objects.shift(self.p))
            },
            Bullet::RotateBullet(rotate_bullet) => {
                unimplemented!();
            },
        }
    }
}

pub struct SimpleBullet {
    p: Point2f,
    v: Point2f,
    a: Point2f,
    graphic_objects: GraphicObjects,
}

impl SimpleBullet {
    pub fn new(p: Point2f, v: Point2f, a: Point2f, graphic_objects: GraphicObjects) -> SimpleBullet {
        println!("new!");
        SimpleBullet {
            p: p,
            v: v,
            a: a,
            graphic_objects: graphic_objects,
        }
    }
    fn get_p(&self) -> Point2f {
        self.p
    }

    fn moving_object_graphics_iter(&self) -> GraphicObjectsIntoIter {
        GraphicObjectsIntoIter::new(self.p, &self.graphic_objects)
    }

    fn tick(&mut self, dt: f32) {
        self.p += self.v * dt;
        self.v += self.a * dt;
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
