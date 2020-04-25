// Point2f is also Vec2f
pub struct Point2f{
    x: f32,
    y: f32,
}

pub struct LineSeg2f{
    begin: Point2f,
    end: Point2f,
}

pub struct Polygen2f{
    PointSeq: Vec<Point2f>,
}
