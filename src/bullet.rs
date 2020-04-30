use lazy_static::lazy_static;

use crate::algebra::{Mat2x2f, Point2f};
use crate::graphic_object::GraphicObjects;

// This struct is static, created by Session::new() only once
pub struct BulletGraphicObjects {
    pub wedge: GraphicObjects,
    pub rectangle: GraphicObjects,
}

lazy_static! {
    pub static ref BULLET_GRAPHIC_OBJECTS: BulletGraphicObjects = {
        BulletGraphicObjects {
            wedge: GraphicObjects::from_strs(vec!["l 1 1 0 1 -10 -5 -10 5 10 0 -10 -5"]),
            rectangle: GraphicObjects::from_strs(vec![
                "l 1 1 0.5 0.5 -2 -10 2 -10 2 10 -2 10 -2 -10",
            ]),
        }
    };
}

pub enum Bullet {
    Simple(SimpleBullet),
    Rotate(RotateBullet),
}

impl Bullet {
    pub fn tick(&mut self, dt: f32) {
        match self {
            Bullet::Simple(simple_bullet) => {
                simple_bullet.tick(dt);
            }
            Bullet::Rotate(rotate_bullet) => {
                unimplemented!();
            }
        }
    }

    pub fn get_shifted_graphic_objects(&self) -> GraphicObjects {
        match self {
            Bullet::Simple(simple_bullet) => simple_bullet.get_shifted_graphic_objects(),
            Bullet::Rotate(rotate_bullet) => {
                unimplemented!();
            }
        }
    }

    pub fn get_p(&self) -> Option<Point2f> {
        match self {
            Bullet::Simple(simple_bullet) => simple_bullet.get_p(),
            Bullet::Rotate(rotate_bullet) => unimplemented!(),
        }
    }

    pub fn get_last_p(&self) -> Option<Point2f> {
        match self {
            Bullet::Simple(simple_bullet) => simple_bullet.get_last_p(),
            Bullet::Rotate(rotate_bullet) => {
                unimplemented!();
            }
        }
    }

    pub fn get_r(&self) -> f32 {
        match self {
            Bullet::Simple(simple_bullet) => simple_bullet.get_r(),
            Bullet::Rotate(rotate_bullet) => {
                unimplemented!();
            }
        }
    }
}

pub struct SimpleBullet {
    p: Point2f,
    last_p: Option<Point2f>,
    v: Point2f,
    a: Point2f,
    r: f32, // radius
    graphic_objects: GraphicObjects,
}

impl SimpleBullet {
    pub fn new(
        p: Point2f,
        v: Point2f,
        a: Point2f,
        r: f32,
        graphic_objects: GraphicObjects,
    ) -> SimpleBullet {
        SimpleBullet {
            p: p,
            last_p: None,
            v: v,
            a: a,
            r: r,
            graphic_objects: graphic_objects,
        }
    }

    fn get_p(&self) -> Option<Point2f> {
        Some(self.p)
    }

    fn get_last_p(&self) -> Option<Point2f> {
        self.last_p
    }

    fn get_r(&self) -> f32 {
        self.r
    }

    fn get_shifted_graphic_objects(&self) -> GraphicObjects {
        self.graphic_objects.shift(self.p)
    }

    fn tick(&mut self, dt: f32) {
        self.last_p = Some(self.p);
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
