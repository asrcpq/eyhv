// Moving objects

use crate::algebra;
use crate::graphicObjects::GraphicObjects;

struct MovingObject{
    // Dynamic
    pos: algebra::Point2f,
    delta_pos: algebra::Point2f,

    // Static
    image: GraphicObjects,
}

pub struct Player{
    object: MovingObject,
}
