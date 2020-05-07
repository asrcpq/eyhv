use std::collections::VecDeque;

use lazy_static::lazy_static;
use rand::Rng;
use rand::SeedableRng;

use crate::algebra::{Mat2x2f, Point2f};
use crate::bullet::{bullet_graphic_objects, Bullet, SimpleBullet};
use crate::cannon::{CannonControllerInterface, CannonGeneratorInterface};
use crate::random_tools::simple_try;

const TRY_TIMES: u32 = 10;

#[derive(Clone)]
pub struct LaserSlicer {
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
    theta: Vec<f32>,
    // has an uninitialized option
    rng: Option<rand_pcg::Pcg64Mcg>,

    bullet_speed: f32,
    count: u32,

    // status
    switch: bool, // on/off
}

impl CannonGeneratorInterface for LaserSlicer {
    fn generate(seed: u64, difficulty: f32, correlation: f32) -> LaserSlicer {
        // difficulty expression
        // difficulty = fire_duration * (bullet_speed / fire_interval)
        // fire_freq = fd(cd * (0.2 - 1)) / cd(1 - 3) / fi(infer)
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        let cycle_duration: f32 = rng.gen_range(2., 4.);
        let result = simple_try(
            TRY_TIMES,
            |x| x[0] * x[1].powi(2) * x[2],
            vec![(0.01, 0.9), (0.01, 3.), (1., 4.)], // 0.05-40
            correlation,
            difficulty,
            rng.gen::<u64>(),
        );
        let (fire_duration, bs_ff, count) = (cycle_duration * result[0], result[1], result[2] as u32);
        let mut bullet_speed = bs_ff.sqrt();
        let fire_interval = 0.05 * bullet_speed / bs_ff;
        bullet_speed *= 600.;
        LaserSlicer {
            p: Point2f::new(),
            fire_duration,
            cycle_duration,
            fire_interval,
            fire_cd: fire_interval,
            theta: vec![0.; count as usize], // uninitialized
            rng: Some(rand_pcg::Pcg64Mcg::seed_from_u64(seed)),
            switch: true,
            bullet_speed,
            count,
            // theta in updated when entering fire phase
            phase_timer: cycle_duration,
        }
    }
}

impl CannonControllerInterface for LaserSlicer {
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

    fn tick(&mut self, host_p: Point2f, _: Point2f, mut dt: f32) -> VecDeque<Box<dyn Bullet>> {
        let mut bullet_queue = VecDeque::new();
        if !self.switch {
            return bullet_queue;
        }
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
                    lazy_static! {
                        static ref RANGE_CONST: f32 = (std::f32::consts::PI / 2.).cbrt();
                    }
                    for i in 0..self.count as usize {
                        self.theta[i] = self
                            .rng
                            .as_mut()
                            .unwrap()
                            .gen_range(-*RANGE_CONST, *RANGE_CONST)
                            .powf(3.)
                            + std::f32::consts::FRAC_PI_2;
                    }
                    self.phase_timer = 0.;
                    self.fire_cd = self.fire_interval;
                }
            }
            while self.phase_timer < self.fire_duration {
                if self.fire_cd > dt {
                    self.fire_cd -= dt;
                    self.phase_timer += dt;
                    break 'cycle bullet_queue;
                }
                dt -= self.fire_cd;
                for i in 0..self.count as usize {
                    let normed_vec2f = Point2f::from_theta(self.theta[i]);
                    bullet_queue.push_back(Box::new(SimpleBullet::new(
                        self.p + host_p,
                        normed_vec2f * self.bullet_speed,
                        Point2f::new(),
                        dt,
                        BULLET_RADIUS,
                        bullet_graphic_objects::LASER_SOLID_BAR
                            .rotate(Mat2x2f::from_normed_vec2f(normed_vec2f)),
                    )));
                }
                self.fire_cd = self.fire_interval;
            }
        }
    }

    fn set_p(&mut self, p: Point2f) {
        self.p = p;
    }

    fn set_rng(&mut self, seed: u64) {
        self.rng = Some(rand_pcg::Pcg64Mcg::seed_from_u64(seed));
    }
}
