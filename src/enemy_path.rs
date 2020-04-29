use crate::algebra::Point2f;
use crate::window_rect::WINDOW_RECT;

pub enum EnemyPath {
    // Straight down with given x
    Straight(StraightDown),
}

impl EnemyPath {
    pub fn tick(&mut self, dt: f32) -> Option<f32> {
        match self {
            EnemyPath::Straight(enemy_path) => {
                enemy_path.tick(dt)
            }
        }
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

    // return None if path ends
    pub fn tick(&mut self, dt: f32) -> Option<f32> {
        self.timer += dt;
        let y = self.vy * self.timer;
        if y > WINDOW_RECT.rd.y {
            None
        } else {
            Some(y)
        }
    }
}
