use crate::algebra::{Point2f, Circle2f};

pub trait CollisionPipeInterface {
    type Object;

    fn push(&mut self, object: Self::Object);
    fn pop(&mut self) -> Option<Self::Object>;
    fn len(&self) -> usize;
}
