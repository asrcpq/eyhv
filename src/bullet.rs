use dyn_clone::DynClone;

use crate::algebra::{Mat2x2f, Point2f};
use crate::graphic_object::GraphicObjects;

pub mod bullet_graphic_objects {
    use crate::graphic_object::GraphicObjects;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref WEDGE: GraphicObjects =
            GraphicObjects::from_strs(vec!["l 1 1 0 1 -10 -5 -10 5 10 0 -10 -5"]);
        pub static ref RECTANGLE: GraphicObjects =
            GraphicObjects::from_strs(vec!["l 1 1 0.5 0.5 -2 -10 2 -10 2 10 -2 10 -2 -10"]);
        pub static ref DIAMOND: GraphicObjects =
            GraphicObjects::from_strs(vec!["l 0.5 0.5 1 1 -8 0 0 6 8 0 0 -6 -8 0"]);
    }
}

pub trait Bullet: DynClone {
    fn tick(&mut self, dt: f32);
    fn get_shifted_graphic_objects(&self) -> GraphicObjects;
    fn get_p(&self) -> Option<Point2f>;
    fn get_last_p(&self) -> Option<Point2f>;
    fn get_r(&self) -> f32;
}

dyn_clone::clone_trait_object!(Bullet);

#[derive(Clone)]
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
}

impl Bullet for SimpleBullet {
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

#[derive(Clone)]
pub struct RotateBullet {
    p: Point2f,
    last_p: Option<Point2f>,
    v: Point2f,
    a: Point2f,
    r: f32,
    // theta is rotate angle, not direction pointing!
    theta: f32,
    rotate_matrix: Mat2x2f,
    graphic_objects: GraphicObjects,
}

impl RotateBullet {
    pub fn new(
        p: Point2f,
        v: Point2f,
        a: Point2f,
        r: f32,
        rotate_matrix: Mat2x2f,
        graphic_objects: GraphicObjects,
    ) -> RotateBullet {
        RotateBullet {
            p: p,
            last_p: None,
            v: v,
            a: a,
            r: r,
            theta: 0.,
            rotate_matrix: rotate_matrix,
            graphic_objects: graphic_objects,
        }
    }
}

impl Bullet for RotateBullet {
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
        // rotate angle is not reproducible
        // because no one cares
        self.graphic_objects = self.graphic_objects.rotate(self.rotate_matrix);
    }
}
