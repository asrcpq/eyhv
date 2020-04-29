use lazy_static::lazy_static;

use std::collections::VecDeque;

use crate::graphic_object::GraphicObjects;
use crate::enemy_path;
use crate::algebra::Point2f;
use crate::bullet::Bullet;

// This struct is static, created by Session::new() only once
#[derive(Clone)]
pub struct EnemyGraphicObjects {
    pub dummy: GraphicObjects,
}

lazy_static! {
    pub static ref ENEMY_GRAPHIC_OBJECTS: EnemyGraphicObjects = {
        EnemyGraphicObjects {
            dummy: GraphicObjects::from_strs(
                vec!["l 1 1 1 1 -10 -10 -10 10 10 10 10 -10 -10 -10"]
            ),
        }
    };
}

pub enum EnemyTickReturnOption {
    Normal(VecDeque<Bullet>),
    Destroyed,
    Removed,
}

pub enum Enemy {
    Dummy(DummyEnemy),
}

impl Enemy {
    pub fn tick(&mut self, dt: f32) -> EnemyTickReturnOption {
        match self {
            Enemy::Dummy(enemy) => {
                enemy.tick(dt)
            }
        }
    }

    pub fn get_shifted_graphic_objects(&self) -> GraphicObjects {
        match self {
            Enemy::Dummy(enemy) => {
                enemy.get_shifted_graphic_objects()
            }
        }
    }
}

pub struct DummyEnemy {
    p: Point2f,
    path: enemy_path::EnemyPath,

    graphic_objects: GraphicObjects,
}

impl DummyEnemy {
    pub fn new() -> DummyEnemy {
        DummyEnemy {
            // this p=(0, 0) should never be rendered!
            p: Point2f::new(),
            path: enemy_path::EnemyPath::Straight(
                enemy_path::StraightDown::new(250., 50.)
            ),
            graphic_objects: ENEMY_GRAPHIC_OBJECTS.dummy.clone(),
        }
    }

    fn tick(&mut self, dt: f32) -> EnemyTickReturnOption {
        match self.path.tick(dt) {
            // update p first to prevent displaying (0, 0)
            None => return EnemyTickReturnOption::Removed,
            Some(point2f) => {
               self.p = point2f;
            },
        }

        // Fire here
        EnemyTickReturnOption::Normal(VecDeque::new())
    }

    pub fn get_shifted_graphic_objects(&self) -> GraphicObjects {
        self.graphic_objects.shift(self.p)
    }
}
