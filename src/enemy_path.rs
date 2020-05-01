use dyn_clone::DynClone;

use crate::algebra::Point2f;
use crate::window_rect::WINDOW_RECT;

pub trait EnemyPath: DynClone {
    // return None if path ends
    fn tick(&mut self, dt: f32) -> Option<Point2f>;
}

dyn_clone::clone_trait_object!(EnemyPath);

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
}

impl EnemyPath for StraightDown {
    fn tick(&mut self, dt: f32) -> Option<Point2f> {
        self.timer += dt;
        let y = self.vy * self.timer;
        if y > WINDOW_RECT.rd.y {
            None
        } else {
            Some(Point2f::from_floats(self.x, y))
        }
    }
}
