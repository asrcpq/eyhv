// Moving objects

use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObject, GraphicObjects};

// return shifted graphics
// So we just need to move the graphic object
pub struct MovingObjectGraphicsIter<'a> {
    p: Point2f,
    graphic_objects: &'a GraphicObjects,
    id: usize,
}

impl<'a> MovingObjectGraphicsIter<'a> {
    // p is position of moving object
    pub fn new(p: Point2f, graphic_objects: &'a GraphicObjects) -> MovingObjectGraphicsIter {
        MovingObjectGraphicsIter {
            p: p,
            graphic_objects: graphic_objects,
            id: 0,
        }
    }
}

impl<'a> Iterator for MovingObjectGraphicsIter<'a> {
    type Item = GraphicObject;

    fn next(&mut self) -> Option<GraphicObject> {
        match self.graphic_objects.get(self.id) {
            Some(graphic_object) => {
                self.id += 1;
                Some(graphic_object.shift(self.p))
            }
            None => None,
        }
    }
}

pub trait MovingObject {
    fn get_p(&self) -> Point2f;
    fn moving_object_graphics_iter(&self) -> MovingObjectGraphicsIter;
}
