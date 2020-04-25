// Moving objects

use crate::algebra;
use crate::graphic_objects::GraphicObjects;

struct MovingObject {
    // Dynamic
    p: algebra::Point2f,
    dp: algebra::Point2f,

    // Static
    image: GraphicObjects,
}

impl MovingObject {
    pub fn set_dp(&mut self, dp: algebra::Point2f) {
        self.dp = dp;
    }

    pub fn update_p(&mut self) {
        self.p += self.dp;
    }
}

pub struct Player {
    object: MovingObject,

    // params
    speed: f32, // per second
}

impl Player {
    pub fn set_dp(&mut self, dp: algebra::Point2f) {}
}
