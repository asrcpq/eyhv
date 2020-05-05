use std::collections::VecDeque;

mod player_locker;
mod rotor;
mod shotgun;
mod laser_locker;
mod laser_slicer;
pub mod simple_cannon;

use player_locker::PlayerLocker;
use rotor::Rotor;
use shotgun::Shotgun;
use laser_locker::LaserLocker;
use laser_slicer::LaserSlicer;
pub use simple_cannon::SimpleCannon;

use dyn_clone::DynClone;
use rand::Rng;
use rand::SeedableRng;

use crate::algebra::Point2f;
use crate::bullet::Bullet;

pub trait CannonControllerInterface: DynClone {
    // once a cannon is turned off, it immediately resets the state of itself
    // static implementation
    fn switch(&mut self, switch: bool);
    fn tick(&mut self, host_p: Point2f, player_p: Point2f, dt: f32) -> VecDeque<Box<dyn Bullet>>;
    fn set_p(&mut self, p: Point2f);

    // for enemy prototype
    fn set_rng(&mut self, _: u64) {}
}

dyn_clone::clone_trait_object!(CannonControllerInterface);

trait CannonGeneratorInterface {
    fn generate(seed: u64, difficulty: f32, correlation: f32) -> Self;
}

pub fn random_mapper(seed: u64, difficulty: f32) -> Box<dyn CannonControllerInterface> {
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
    let correlation: f32 = 0.5;
    const CANNON_TYPES: u32 = 5;
    match rng.gen_range(0, CANNON_TYPES) {
        0 => Box::new(PlayerLocker::generate(rng.gen::<u64>(), difficulty, correlation)),
        1 => Box::new(Rotor::generate(rng.gen::<u64>(), difficulty, correlation)),
        2 => Box::new(Shotgun::generate(rng.gen::<u64>(), difficulty, correlation)),
        3 => Box::new(LaserLocker::generate(rng.gen::<u64>(), difficulty, correlation)),
        4 => Box::new(LaserSlicer::generate(rng.gen::<u64>(), difficulty, correlation)),
        _ => unreachable!(),
    }
}
