use crate::algebra::Point2f;

pub trait CollisionPipeInterface {
    type Object;

    fn push(&mut self, object: Self::Object);
    fn pop(&mut self) -> Option<Self::Object>;
    fn len(&self) -> usize;
}

pub trait ObjectPositionInterface {
    fn get_p(&self) -> Option<Point2f>;
    fn get_last_p(&self) -> Option<Point2f>;
}
