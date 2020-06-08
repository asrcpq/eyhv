use std::collections::VecDeque;

use rand::Rng;
use rand::SeedableRng;

use crate::algebra::{Mat2x2f, Point2f};
use crate::bullet::{bullet_graphic_objects, Bullet, SimpleBullet};
use crate::cannon::{CannonControllerInterface, CannonGeneratorInterface};
use crate::random_tools::simple_try;

const TRY_TIMES: u32 = 10;

#[derive(Clone)]
pub struct Ring {
    // relative to moving object
    p: Point2f,

    // bullet shooted during fire phase
    fire_interval: f32,

    // timer between intervals
    fire_cd: f32,
    count: u32,
    bullet_speed: f32,

    rng: Option<rand_pcg::Pcg64Mcg>,

    // status
    switch: bool, // on/off
}

impl CannonGeneratorInterface for Ring {
    fn generate(seed: u64, difficulty: f32, correlation: f32) -> Ring {
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        // difficulty = bullet_speed / fire_interval * count
        let generated = simple_try(
            TRY_TIMES,
            |x| x[0] / x[1] * x[2],
            vec![(200., 600.), (2.0, 0.1), (7., 90.)],
            correlation,
            difficulty,
            rng.gen::<u64>(),
        );
        let (bullet_speed, fire_interval, count) =
            (generated[0], generated[1], generated[2] as u32);
        Ring {
            p: Point2f::new(),
            fire_interval,
            fire_cd: 0.,
            count,
            bullet_speed,
            rng: None,
            switch: true,
        }
    }
}

impl CannonControllerInterface for Ring {
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
        const BULLET_RADIUS: f32 = 3.;
        loop {
            if self.fire_cd > dt {
                self.fire_cd -= dt;
                break bullet_queue;
            }
            dt -= self.fire_cd;
            let d_theta = 2. * std::f32::consts::PI / self.count as f32;
            let mut theta = self.rng.as_mut().unwrap().gen_range(0., d_theta);
            for _ in 0..self.count {
                theta += d_theta;
                let normed_vec2f = Point2f::from_theta(theta);
                bullet_queue.push_back(Box::new(SimpleBullet::new(
                    self.p + host_p,
                    normed_vec2f * self.bullet_speed,
                    Point2f::new(),
                    dt,
                    BULLET_RADIUS,
                    bullet_graphic_objects::FLAT_HEXAGON
                        .clone()
                        .rotate(Mat2x2f::from_normed_vec2f(normed_vec2f)),
                )));
            }
            self.fire_cd = self.fire_interval;
        }
    }

    fn set_p(&mut self, p: Point2f) {
        self.p = p;
    }

    fn set_rng(&mut self, seed: u64) {
        self.rng = Some(rand_pcg::Pcg64Mcg::seed_from_u64(seed));
    }
}
