use crate::algebra::Point2f;
use crate::window_rect::WINDOW_RECT;

#[derive(Clone)]
pub enum EnemyPath {
    // Straight down with given x
    Straight(StraightDown),
}

impl EnemyPath {
    pub fn tick(&mut self, dt: f32) -> Option<Point2f> {
        match self {
            EnemyPath::Straight(enemy_path) => enemy_path.tick(dt),
        }
    }
}

#[derive(Clone)]
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

    // return None if path ends
    pub fn tick(&mut self, dt: f32) -> Option<Point2f> {
        self.timer += dt;
        let y = self.vy * self.timer;
        if y > WINDOW_RECT.rd.y {
            None
        } else {
            Some(Point2f::from_floats(self.x, y))
        }
    }
}
