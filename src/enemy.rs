use lazy_static::lazy_static;

use std::collections::VecDeque;

use crate::graphic_object::GraphicObjects;
use crate::enemy_path;
use crate::algebra::Point2f;
use crate::bullet::Bullet;
use crate::collision_pipe_interface::ObjectPositionInterface;

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

impl ObjectPositionInterface for Enemy {
    fn get_p(&self) -> Option<Point2f> {
        match self {
            Enemy::Dummy(enemy) => {
                enemy.get_p()
            }
        }
    }

    fn get_last_p(&self) -> Option<Point2f> {
        match self {
            Enemy::Dummy(enemy) => {
                enemy.get_last_p()
            }
        }
    }
}

pub struct DummyEnemy {
    p: Option<Point2f>,
    last_p: Option<Point2f>,
    path: enemy_path::EnemyPath,

    graphic_objects: GraphicObjects,
}

impl DummyEnemy {
    pub fn new() -> DummyEnemy {
        DummyEnemy {
            p: None,
            last_p: None,
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
                self.last_p = self.p;
                self.p = Some(point2f);
            },
        }

        // Fire here
        EnemyTickReturnOption::Normal(VecDeque::new())
    }

    fn get_shifted_graphic_objects(&self) -> GraphicObjects {
        self.graphic_objects.shift(self.p.unwrap())
    }

    fn get_p(&self) -> Option<Point2f> {
        self.p
    }

    fn get_last_p(&self) -> Option<Point2f> {
        self.last_p
    }
}
