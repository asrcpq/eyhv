use lazy_static::lazy_static;

use std::collections::VecDeque;

use crate::algebra::{Circle2f, Point2f};
use crate::bullet::Bullet;
use crate::cannon::{CannonControllerInterface, CannonGeneratorInterface};
use crate::cannon::{PlayerLocker, SimpleCannon};
use crate::enemy_path;
use crate::graphic_object::GraphicObjects;

// This struct is static, created by Session::new() only once
#[derive(Clone)]
pub struct EnemyGraphicObjects {
    pub small1: GraphicObjects,
    pub dummy: GraphicObjects,
}

lazy_static! {
    pub static ref ENEMY_GRAPHIC_OBJECTS: EnemyGraphicObjects = {
        EnemyGraphicObjects {
            small1: GraphicObjects::from_strs(vec![
                "l 0.3 0.5 0.5 1. -0.5 0.5 -0.5 -0.5 0.5 -0.5 0.5 0.5 -0.5 0.5",
                "l 0.7 0.0 0.2 0.6 -0.5 0.3 -1.5 1. -1.5 -1. -0.5 -0.3",
                "l 0.7 0.0 0.2 0.6 0.5 0.3 1.5 1. 1.5 -1. 0.5 -0.3",
                ]).zoom(10.),
            dummy: GraphicObjects::from_strs(vec!["l 1 1 1 1 -20 -20 -20 20 20 20 20 -20 -20 -20"]),
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
    pub fn tick(&mut self, dt: f32, player_p: Point2f) -> EnemyTickReturnOption {
        match self {
            Enemy::Dummy(enemy) => enemy.tick(dt, player_p),
        }
    }

    pub fn get_shifted_graphic_objects(&self) -> GraphicObjects {
        match self {
            Enemy::Dummy(enemy) => enemy.get_shifted_graphic_objects(),
        }
    }

    pub fn get_p(&self) -> Option<Point2f> {
        match self {
            Enemy::Dummy(enemy) => enemy.get_p(),
        }
    }

    pub fn get_last_p(&self) -> Option<Point2f> {
        match self {
            Enemy::Dummy(enemy) => enemy.get_last_p(),
        }
    }

    pub fn get_hitboxes(&self) -> &Vec<Circle2f> {
        match self {
            Enemy::Dummy(enemy) => enemy.get_hitboxes(),
        }
    }
}

pub struct DummyEnemy {
    p: Option<Point2f>,
    last_p: Option<Point2f>,
    path: enemy_path::EnemyPath,

    cannon: PlayerLocker,

    graphic_objects: GraphicObjects,
    hitboxes: Vec<Circle2f>,
}

impl DummyEnemy {
    pub fn new() -> DummyEnemy {
        DummyEnemy {
            p: None,
            last_p: None,
            path: enemy_path::EnemyPath::Straight(enemy_path::StraightDown::new(250., 50.)),
            cannon: PlayerLocker::generate(Point2f::from_floats(0., 0.), 12345, 0.2),
            graphic_objects: ENEMY_GRAPHIC_OBJECTS.small1.clone(),
            hitboxes: vec![Circle2f::from_floats(0., 0., 20.)],
        }
    }

    fn tick(&mut self, dt: f32, player_p: Point2f) -> EnemyTickReturnOption {
        match self.path.tick(dt) {
            // update p first to prevent displaying (0, 0)
            None => return EnemyTickReturnOption::Removed,
            Some(point2f) => {
                self.last_p = self.p;
                self.p = Some(point2f);
            }
        }

        // path is executed before update_theta so unwrap p should be safe
        self.cannon.update_theta(player_p, self.p.unwrap());
        EnemyTickReturnOption::Normal(self.cannon.fire_tick(self.p.unwrap(), dt))
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

    fn get_hitboxes(&self) -> &Vec<Circle2f> {
        &self.hitboxes
    }
}
