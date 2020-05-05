use std::collections::VecDeque;

use crate::algebra::{Circle2f, Point2f};
use crate::bullet::Bullet;
use crate::cannon::CannonControllerInterface;
use crate::enemy_path::EnemyPath;
use crate::graphic_object::GraphicObjects;

mod enemy_graphic_objects {
    use crate::graphic_object::GraphicObjects;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref SMALL1: GraphicObjects = GraphicObjects::from_strs(vec![
            "l 0.3 0.5 0.5 1 -0.5 0.5 -0.5 -0.5 0.5 -0.5 0.5 0.5 -0.5 0.5",
            "l 1 1 0.2 0.8 -0.5 0.3 -1.5 1. -1.5 -1. -0.5 -0.3",
            "p 1 1 1 0.2 -0.5 0.3 -1.5 1 -1.5 -1 -0.5 -0.3",
            "l 1 1 0.2 0.8 0.5 0.3 1.5 1. 1.5 -1. 0.5 -0.3",
            "p 1 1 1 0.2 0.5 0.3 1.5 1 1.5 -1 0.5 -0.3",
        ])
        .zoom(10.);
        pub static ref SMALL2: GraphicObjects = GraphicObjects::from_strs(vec![
            "l 0.3 0.5 0.5 1 0 -1 1 0 0 1 -1 0 0 -1",
            "l 0.3 0.5 0.5 1 0.5 -0.5 2 -1 1 0",
            "l 0.3 0.5 0.5 1 -0.5 -0.5 -2 -1 -1 0",
            "p 0.1 0.1 1 0.2 0 1 1.5 -0.5 -1.5 -0.5",
        ])
        .zoom(10.);
        pub static ref MEDIUM1: GraphicObjects = GraphicObjects::from_strs(vec![
            "l 1 0.2 0.2 1 0.7 -0.6 0.7 0.1 0.2 0.8 -0.2 0.8 -0.7 0.1 -0.7 -0.6 0.7 -0.6",
            "l 1 1 1 0.5 0.6 -0.1 1.2 -0.1 1.2 -1 3 -1 3 1 1.2 1 1.2 0.1 0.6 0.1",
            "l 1 1 1 0.5 -0.6 -0.1 -1.2 -0.1 -1.2 -1 -3 -1 -3 1 -1.2 1 -1.2 0.1 -0.6 0.1",
            "p 1 0.2 0.3 0.3 3 -1 3.5 -1 3.5 1 3 1",
            "p 1 0.2 0.3 0.3 -3 -1 -3.5 -1 -3.5 1 -3 1",
        ]).zoom(15.);
        pub static ref LARGE1: GraphicObjects = GraphicObjects::from_strs(vec![
            "l 0.8 0.8 1 0.8 2.4 0.5 2.4 -1 -2.4 -1 -2.4 0.5 -3.2 1.5 3.2 1.5 2.4 0.5",
            "p 0.8 0.8 1 0.2 2.4 0.5 2.4 -1 -2.4 -1 -2.4 0.5 -3.2 1.5 3.2 1.5",
            "l 1 1 1 0.7 2.4 0 4 0",
            "l 1 1 1 0.7 2.4 0.5 4 0.5",
            "l 1 1 1 0.7 -2.4 0 -4 0",
            "l 1 1 1 0.7 -2.4 0.5 -4 0.5",
            "l 1 0.9 0.7 1 -4 0 -6 -2 -6 -1 -5 0.5 -7 3 -5 3 -4 0.5 -4 0",
            "p 1 0.8 0.6 0.2 -4 0 -6 -2 -6 -1 -5 0.5 -7 3 -5 3 -4 0.5",
            "l 1 0.9 0.7 1 4 0 6 -2 6 -1 5 0.5 7 3 5 3 4 0.5 4 0",
            "p 1 0.8 0.6 0.2 4 0 6 -2 6 -1 5 0.5 7 3 5 3 4 0.5",
            "l 1 1 1 1 3.2 1.5 3.2 2.5 -3.2 2.5 -3.2 1.5 3.2 1.5",
        ]).zoom(15.);
    }
}

pub enum EnemyTickReturnOption {
    Normal(VecDeque<Box<dyn Bullet>>),
    Removed,
}

#[derive(Clone)]
pub struct Enemy {
    p: Option<Point2f>,
    last_p: Option<Point2f>,
    path: EnemyPath,
    speed: f32,
    life: f32,
    cannons: Vec<Box<dyn CannonControllerInterface>>,
    graphic_objects: GraphicObjects,
    hitboxes: Vec<Circle2f>,
}

pub mod enemy_prototype {
    use super::enemy_graphic_objects;
    use crate::algebra::{Circle2f, Point2f};
    use crate::graphic_object::GraphicObjects;
    use lazy_static::lazy_static;

    #[derive(Clone)]
    pub struct EnemyPrototype {
        pub speed: f32,
        pub life: f32,
        pub cannon_pits: Vec<Vec<Point2f>>,
        pub hitboxes: Vec<Circle2f>,
        pub graphic_objects_options: Vec<GraphicObjects>,
    }

    lazy_static! {
        pub static ref SMALL: EnemyPrototype = EnemyPrototype {
            speed: 1.,
            life: 3.,
            cannon_pits: vec![vec![Point2f::new()]],
            hitboxes: vec![Circle2f::from_floats(0., 0., 20.)],
            graphic_objects_options: vec![
                enemy_graphic_objects::SMALL1.clone(),
                enemy_graphic_objects::SMALL2.clone(),
            ],
        };
        pub static ref MEDIUM: EnemyPrototype = EnemyPrototype {
            speed: 0.5,
            life: 15.,
            cannon_pits: vec![
            vec![
                Point2f::from_floats(-30., 0.),
                Point2f::from_floats(30., 0.),
            ], vec![
                Point2f::from_floats(-30., 0.),
                Point2f::from_floats(30., 0.),
            ]],
            hitboxes: vec![
                Circle2f::from_floats(18., 0., 20.),
                Circle2f::from_floats(-18., 0., 20.),
            ],
            graphic_objects_options: vec![
                enemy_graphic_objects::MEDIUM1.clone(),
            ],
        };
        pub static ref LARGE1: EnemyPrototype = EnemyPrototype {
            speed: 0.4,
            life: 100.,
            cannon_pits: vec![
            vec![
                Point2f::from_floats(-45., 30.),
                Point2f::from_floats(-15., 30.),
                Point2f::from_floats(15., 30.),
                Point2f::from_floats(45., 30.),
            ], vec![
                Point2f::from_floats(-75., 20.),
                Point2f::from_floats(75., 20.),
            ], vec![
                Point2f::from_floats(0., 0.),
            ]],
            hitboxes: vec![
                Circle2f::from_floats(60., 0., 30.),
                Circle2f::from_floats(-60., 0., 30.),
                Circle2f::from_floats(0., 15.,  30.),
            ],
            graphic_objects_options: vec![
                enemy_graphic_objects::LARGE1.clone(),
            ],
        };
    }
}

impl Enemy {
    pub fn new(
        path: EnemyPath,
        speed: f32,
        life: f32,
        cannons: Vec<Box<dyn CannonControllerInterface>>,
        graphic_objects: GraphicObjects,
        hitboxes: Vec<Circle2f>,
    ) -> Enemy {
        Enemy {
            p: None,
            last_p: None,
            path,
            life,
            speed,
            cannons,
            graphic_objects,
            hitboxes,
        }
    }

    pub fn tick(&mut self, dt: f32, player_p: Point2f) -> EnemyTickReturnOption {
        match self.path.tick(dt * self.speed) {
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

    // return survive status
    pub fn damage(&mut self, damage: f32) -> bool {
        self.life -= damage;
        self.life > 0.
    }
}
