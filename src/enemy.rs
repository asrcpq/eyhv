use lazy_static::lazy_static;

use std::collections::VecDeque;

use crate::algebra::{Circle2f, Point2f};
use crate::bullet::Bullet;
use crate::cannon::{CannonControllerInterface, CannonGeneratorInterface};
use crate::cannon::{PlayerLocker, EnemyCannon};
use crate::enemy;
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

#[derive(Clone)]
pub struct Enemy {
    p: Option<Point2f>,
    last_p: Option<Point2f>,
    path: enemy_path::EnemyPath,

    cannon: Vec<Box<CannonControllerInterface>>,

    graphic_objects: GraphicObjects,
    hitboxes: Vec<Circle2f>,
}

impl Enemy {
    pub fn new_dummy() -> Enemy {
        Enemy {
            p: None,
            last_p: None,
            path: enemy_path::EnemyPath::Straight(enemy_path::StraightDown::new(250., 50.)),
            cannon: vec![Box::new(PlayerLocker::generate(Point2f::from_floats(0., 0.), 12345, 0.2))],
            graphic_objects: enemy::ENEMY_GRAPHIC_OBJECTS.small1.clone(),
            hitboxes: vec![Circle2f::from_floats(0., 0., 20.)],
        }
    }
    pub fn tick(&mut self, dt: f32, player_p: Point2f) -> EnemyTickReturnOption {
        match self.path.tick(dt) {
            // update p first to prevent displaying (0, 0)
            None => return EnemyTickReturnOption::Removed,
                 Some(point2f) => {
                     self.last_p = self.p;
                     self.p = Some(point2f);
                 }
        }

        let mut bullet_queue = VecDeque::new();
        // path is executed before update_theta so unwrap p should be safe
        for cannon in self.cannon.iter_mut() {
            bullet_queue.extend(cannon.tick(
                self.p.unwrap(),
                player_p,
                dt,
            ));
        }
        EnemyTickReturnOption::Normal(bullet_queue)
    }

    pub fn get_shifted_graphic_objects(&self) -> GraphicObjects {
        self.graphic_objects.shift(self.p.unwrap())
    }

    pub fn get_p(&self) -> Option<Point2f> {
        self.p
    }

    pub fn get_last_p(&self) -> Option<Point2f> {
        self.last_p
    }

    pub fn get_hitboxes(&self) -> &Vec<Circle2f> {
        &self.hitboxes
    }
}
