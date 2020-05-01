use std::collections::VecDeque;

use crate::algebra::{Circle2f, Point2f};
use crate::bullet::Bullet;
use crate::cannon::{CannonControllerInterface, CannonGeneratorInterface};
use crate::cannon::{EnemyCannon, PlayerLocker};
use crate::enemy;
use crate::enemy_path::EnemyPath;
use crate::graphic_object::GraphicObjects;

mod enemy_graphic_objects {
    use crate::graphic_object::GraphicObjects;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref SMALL1: GraphicObjects = GraphicObjects::from_strs(vec![
            "l 0.3 0.5 0.5 1. -0.5 0.5 -0.5 -0.5 0.5 -0.5 0.5 0.5 -0.5 0.5",
            "l 0.7 0.0 0.2 0.6 -0.5 0.3 -1.5 1. -1.5 -1. -0.5 -0.3",
            "l 0.7 0.0 0.2 0.6 0.5 0.3 1.5 1. 1.5 -1. 0.5 -0.3",
        ])
        .zoom(10.);
    }
}

pub enum EnemyTickReturnOption {
    Normal(VecDeque<Bullet>),
    Removed,
}

#[derive(Clone)]
pub struct Enemy {
    p: Option<Point2f>,
    last_p: Option<Point2f>,
    path: EnemyPath,

    cannons: Vec<Box<dyn CannonControllerInterface>>,

    graphic_objects: GraphicObjects,
    hitboxes: Vec<Circle2f>,
}

impl Enemy {
    pub fn new_small(path: EnemyPath, cannons: Vec<Box<dyn CannonControllerInterface>>) -> Enemy {
        Enemy {
            p: None,
            last_p: None,
            path: path,
            cannons: cannons,
            graphic_objects: enemy_graphic_objects::SMALL1.clone(),
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
        for cannon in self.cannons.iter_mut() {
            bullet_queue.extend(cannon.tick(self.p.unwrap(), player_p, dt));
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
