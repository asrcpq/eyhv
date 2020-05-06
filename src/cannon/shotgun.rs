use std::collections::VecDeque;

use rand::Rng;
use rand::SeedableRng;

use crate::algebra::{Mat2x2f, Point2f};
use crate::bullet::{bullet_graphic_objects, Bullet, RotateBullet};
use crate::random_tools::simple_try;
use crate::cannon::{CannonGeneratorInterface, CannonControllerInterface};

const TRY_TIMES: u32 = 10;

#[derive(Clone)]
pub struct Shotgun {
    // relative to moving object
    p: Point2f,

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
    open_angle: f32,
    count: u32,
    rng: Option<rand_pcg::Pcg64Mcg>,

    bullet_speed: f32,

    // status
    switch: bool, // on/off
}

impl CannonGeneratorInterface for Shotgun {
    fn generate(seed: u64, difficulty: f32, correlation: f32) -> Shotgun {
        // difficulty expression
        // difficulty = fire_duration * count * (bullet_speed / fire_interval)
        // fire_freq = fd(cd * (0.2 - 1)) / cd(1 - 3) / fi(infer)
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        // cn * bs_ff^2
        let generated = simple_try(
            TRY_TIMES,
            |x| x[0] * x[1].powi(2),
            vec![(1., 7.), (0.01, 2.)],
            correlation,
            difficulty,
            rng.gen::<u64>(),
        );
        let (count, bs_ff) = (generated[0], generated[1]);
        let bs_ff_k = rng.gen_range(0.8, 1.2);
        let mut bullet_speed = (bs_ff * bs_ff_k).sqrt();
        let fire_interval = 0.3 * bullet_speed / bs_ff;
        bullet_speed *= 600.;
        let open_angle: f32 = rng.gen_range(-1f32, 1.2f32).exp();
        Shotgun {
            p: Point2f::new(),
            fire_interval,
            fire_cd: fire_interval,
            theta: 0., // uninitialized
            open_angle,
            count: count as u32,
            rng: None,
            switch: true,
            bullet_speed,
            phase_timer: 0.,
        }
    }
}

impl Shotgun {
    fn update_theta(&mut self, player_p: Point2f, self_p: Point2f) {
        // r points to player
        let r = player_p - self_p;
        self.theta = r.y.atan2(r.x);
    }
}

impl CannonControllerInterface for Shotgun {
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
        let rotate_matrix: Mat2x2f = Mat2x2f::from_theta(0.1);
        loop {
            if self.fire_cd > dt {
                self.fire_cd -= dt;
                self.phase_timer += dt;
                break bullet_queue;
            }
            dt -= self.fire_cd;
            for _ in 0..self.count {
                let normed_vec2f = Point2f::from_theta(self.rng.as_mut().unwrap().gen_range(
                    self.theta - self.open_angle / 2.,
                    self.theta + self.open_angle / 2.,
                ));
                let bullet_speed =
                    normed_vec2f *
                    self.bullet_speed *
                    self.rng.as_mut().unwrap().gen_range(0.8, 1.2);
                bullet_queue.push_back(Box::new(RotateBullet::new(
                    self.p + host_p,
                    bullet_speed,
                    Point2f::new(),
                    dt,
                    BULLET_RADIUS,
                    rotate_matrix,
                    bullet_graphic_objects::SQUARE
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
