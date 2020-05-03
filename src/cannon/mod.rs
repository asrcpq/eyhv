use std::collections::VecDeque;

mod player_locker;
mod rotor;
mod shotgun;

use player_locker::PlayerLocker;
use rotor::Rotor;
use shotgun::Shotgun;

use dyn_clone::DynClone;
use rand::Rng;
use rand::SeedableRng;

use crate::algebra::Point2f;
use crate::bullet::{bullet_graphic_objects, Bullet, SimpleBullet};

pub trait CannonControllerInterface: DynClone {
    // once a cannon is turned off, it immediately resets the state of itself
    // static implementation
    fn switch(&mut self, switch: bool);
    fn tick(&mut self, host_p: Point2f, player_p: Point2f, dt: f32) -> VecDeque<Box<dyn Bullet>>;

    fn set_p(&mut self, p: Point2f);
}

dyn_clone::clone_trait_object!(CannonControllerInterface);

trait CannonGeneratorInterface {
    fn generate(seed: u64, difficulty: f32) -> Self;
}

pub fn random_mapper(seed: u64, difficulty: f32) -> Box<dyn CannonControllerInterface> {
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
    const CANNON_TYPES: u32 = 3;
    match rng.gen_range(0, CANNON_TYPES) {
        0 => Box::new(PlayerLocker::generate(rng.gen::<u64>(), difficulty)),
        1 => Box::new(Rotor::generate(rng.gen::<u64>(), difficulty)),
        2 => Box::new(Shotgun::generate(rng.gen::<u64>(), difficulty)),
        _ => unreachable!(),
    }
}

// SimpleCannon fires bullets with the same and constant speed
// in the same direction at regular intervals
// It is designed for Player
pub struct SimpleCannon {
    // relative to moving object
    p: Point2f,

    fire_interval: f32,
    fire_cd: f32,

    // for player, -90 deg is facing forward
    v: Point2f,

    switch: bool,
}

impl SimpleCannon {
    pub fn new(p: Point2f, v: Point2f, fi: f32, sw: bool) -> SimpleCannon {
        SimpleCannon {
            p,
            fire_interval: fi,
            // player should not benefit from a rapid fire controller
            fire_cd: fi,
            v,
            switch: sw,
        }
    }
}

impl SimpleCannon {
    pub fn switch(&mut self, switch: bool) {
        if self.switch && !switch {
            self.switch = false;
            self.fire_cd = self.fire_interval;
        } else if switch {
            self.switch = true;
        }
    }

    pub fn tick(&mut self, host_p: Point2f, mut dt: f32) -> VecDeque<Box<dyn Bullet>> {
        const BULLET_RADIUS: f32 = 3.;
        let mut bullet_queue = VecDeque::new();
        if !self.switch {
            return bullet_queue;
        }
        loop {
            if self.fire_cd - dt > 0. {
                self.fire_cd -= dt;
                break bullet_queue;
            } else {
                dt -= self.fire_cd;
                self.fire_cd = self.fire_interval;
                bullet_queue.push_back(Box::new(SimpleBullet::new(
                    self.p + host_p,
                    self.v,
                    Point2f::new(),
                    BULLET_RADIUS,
                    bullet_graphic_objects::RECTANGLE.clone(),
                )));
            }
        }
    }
}
