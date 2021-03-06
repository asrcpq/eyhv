use std::collections::VecDeque;

use rand::Rng;
use rand::SeedableRng;

use crate::algebra::{Mat2x2f, Point2f};
use crate::bullet::{bullet_graphic_objects, Bullet, RotateBullet};
use crate::cannon::{CannonControllerInterface, CannonGeneratorInterface};
use crate::random_tools::simple_try;

use mray::graphic_object::GraphicObjects;

const TRY_TIMES: u32 = 10;

#[derive(Clone)]
pub struct Rotor {
	// relative to moving object
	p: Point2f,

	// bullet shooted during fire phase
	fire_interval: f32,

	// timer between intervals
	fire_cd: f32,

	// rotating angle and speed
	theta: f32,
	omega: f32,
	flipped: bool,

	bullet_speed: f32,
	bullet: GraphicObjects,

	// status
	switch: bool, // on/off
}

impl CannonGeneratorInterface for Rotor {
	fn generate(seed: u64, difficulty: f32, _: f32) -> Rotor {
		let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
		// difficulty = bullet_speed / fire_interval
		let generated = simple_try(
			TRY_TIMES,
			|x| x[0] / x[1],
			vec![(100., 700.), (0.17, 0.02)],
			0.7,
			difficulty,
			rng.gen::<u64>(),
		);
		let (bullet_speed, fire_interval) = (generated[0], generated[1]);
		let omega: f32 = rng.gen_range(0.5, 5.);
		let flipped = rng.gen::<bool>();
		// let theta: f32 = rng.gen_range(0., 2. * std::f32::consts::PI);
		let theta: f32 = if flipped { std::f32::consts::PI } else { 0. };
		Rotor {
			p: Point2f::new(),
			fire_interval,
			fire_cd: fire_interval,
			theta,
			flipped,
			omega,
			bullet_speed,
			bullet: if flipped {
				bullet_graphic_objects::DIAMOND2.clone()
			} else {
				bullet_graphic_objects::DIAMOND.clone()
			},
			switch: true,
		}
	}
}

impl CannonControllerInterface for Rotor {
	#[inline]
	fn switch(&mut self, switch: bool) {
		if self.switch && !switch {
			self.switch = false;
			self.fire_cd = self.fire_interval;
		} else if switch {
			self.switch = true;
		}
	}

	fn tick(&mut self, host_p: Point2f, _: Point2f, mut dt: f32) -> VecDeque<Box<dyn Bullet>> {
		let mut bullet_queue = VecDeque::new();
		if self.flipped {
			self.theta -= self.omega * dt;
		} else {
			self.theta += self.omega * dt;
		}
		const BULLET_RADIUS: f32 = 3.;
		let rotate_matrix: Mat2x2f = Mat2x2f::from_theta(0.1);
		loop {
			if self.fire_cd > dt {
				self.fire_cd -= dt;
				break bullet_queue;
			}
			dt -= self.fire_cd;
			bullet_queue.push_back(Box::new(RotateBullet::new(
				self.p + host_p,
				Point2f::new(),
				Point2f::from_polar(self.bullet_speed, self.theta),
				dt,
				BULLET_RADIUS,
				rotate_matrix,
				self.bullet.clone(),
			)));
			self.fire_cd = self.fire_interval;
		}
	}

	fn set_p(&mut self, p: Point2f) {
		self.p = p;
	}
}
