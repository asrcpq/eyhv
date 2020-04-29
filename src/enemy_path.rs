use crate::algebra::Point2f;

pub enum EnemyPath {
    // Straight down with given x
    Straight(StraightDown),
}

impl EnemyPath {
    pub fn tick(dt: f32) {
    }
}

pub struct StraightDown {
    timer: f32,
    x: f32,
    vy: f32,
}

impl StraightDown {
    pub fn new(x: f32, vy: f32) -> StraightDown {
        StraightDown {
            timer: 0.,
            x: x,
            vy: vy,
        }
    }
}
