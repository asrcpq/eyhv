use crate::algebra;

enum GraphicObject{
    line(algebra::LineSeg2f),
    polygon(algebra::Polygen2f),
}

pub struct GraphicObjects{
    data: Vec<GraphicObject>,
}
