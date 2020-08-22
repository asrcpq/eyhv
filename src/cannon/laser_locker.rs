use std::collections::VecDeque;

use rand::Rng;
use rand::SeedableRng;

use crate::algebra::{Mat2x2f, Point2f};
use crate::bullet::{bullet_graphic_objects, Bullet, SimpleBullet};
use crate::cannon::{CannonControllerInterface, CannonGeneratorInterface};
use crate::random_tools::simple_try;

const TRY_TIMES: u32 = 10;

#[derive(Clone)]
pub struct LaserLocker {
	// relative to moving object
	p: Point2f,

	// Durations, phase = fire + cd
	fire_duration: f32,
	cycle_duration: f32,

	// phase_timer takes value from 0-cycle_duration, and reset
	phase_timer: f32,

	// bullet shooted during fire phase
	fire_interval: f32,

	// timer between intervals
	fire_cd: f32,

	// direction, opening angle and bullet number
	// bullets are uniformly distributed on opening angle
	// and are shooted together
	theta: f32,

	bullet_speed: f32,

	// status
	switch: bool, // on/off
}

impl CannonGeneratorInterface for LaserLocker {
	fn generate(seed: u64, difficulty: f32, correlation: f32) -> LaserLocker {
		// difficulty expression
		// difficulty = fire_duration * (bullet_speed / fire_interval)
		// fire_freq = fd(cd * (0.2 - 1)) / cd(1 - 3) / fi(infer)
		let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
		let cycle_duration: f32 = rng.gen_range(1., 2.);
		// k(fd / cd) * bs_ff^2
		let result = simple_try(
			TRY_TIMES,
			|x| x[0] * x[1].powi(2),
			vec![(0.01, 0.5), (0.02, 3.)], // 0.05-40
			correlation,
			difficulty,
			rng.gen::<u64>(),
		);
		let (fire_duration, bs_ff) = (cycle_duration * result[0], result[1]);
		let mut bullet_speed = bs_ff.sqrt();
		let fire_interval = 0.05 * bullet_speed / bs_ff;
		bullet_speed *= 600.;
		LaserLocker {
			p: Point2f::new(),
			fire_duration,
			cycle_duration,
			fire_interval,
			fire_cd: fire_interval,
			theta: 0., // uninitialized
			switch: true,
			bullet_speed,
			phase_timer: 0.,
		}
	}
}

impl LaserLocker {
	fn update_theta(&mut self, player_p: Point2f, self_p: Point2f) {
		// r points to player
		let r = player_p - self_p - self.p;
		self.theta = r.y.atan2(r.x);
	}
}

impl CannonControllerInterface for LaserLocker {
	#[inline]
	fn switch(&mut self, switch: bool) {
		if self.switch && !switch {
			self.switch = false;
			self.phase_timer = 0.;
			self.fire_cd = self.fire_interval;
		} else if switch {
			self.switch = true;
		}
	}

	fn tick(
		&mut self,
		host_p: Point2f,
		player_p: Point2f,
		mut dt: f32,
	) -> VecDeque<Box<dyn Bullet>> {
		self.update_theta(player_p, host_p);
		let mut bullet_queue = VecDeque::new();
		const BULLET_RADIUS: f32 = 3.;
		'cycle: loop {
			if self.phase_timer > self.fire_duration {
				// note that fire_cd should be re-initialized somewhere
				// (when entering cd phase(here) or when entering fire phase)
				// if phase_timer < self.fire_duration and fire_cd > dt
				// the phase_timer is shifted leaving fire_cd a dangling value
				// if phase_timer has gone out of fire phase

				if self.phase_timer + dt < self.cycle_duration {
					self.phase_timer += dt;
					break 'cycle bullet_queue;
				} else {
					dt -= self.cycle_duration - self.phase_timer;
					self.phase_timer = 0.;
					// self.fire_cd = self.fire_interval;
					// fire immediately to fire 1 bullet at least
					self.fire_cd = 0.;
				}
			}
			while self.phase_timer < self.fire_duration {
				if self.fire_cd > dt {
					self.fire_cd -= dt;
					self.phase_timer += dt;
					break 'cycle bullet_queue;
				}
				dt -= self.fire_cd;
				let normed_vec2f = Point2f::from_theta(self.theta);
				bullet_queue.push_back(Box::new(SimpleBullet::new(
					self.p + host_p,
					normed_vec2f * self.bullet_speed,
					Point2f::new(),
					dt,
					BULLET_RADIUS,
					bullet_graphic_objects::LASER_BAR
						.rotate(Mat2x2f::from_normed_vec2f(normed_vec2f)),
				)));
				self.fire_cd = self.fire_interval;
			}
		}
	}

	fn set_p(&mut self, p: Point2f) {
		self.p = p;
	}
}
