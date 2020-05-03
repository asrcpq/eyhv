use std::collections::VecDeque;

use dyn_clone::DynClone;
use rand::Rng;
use rand::SeedableRng;

use crate::algebra::{Mat2x2f, Point2f};
use crate::bullet::{bullet_graphic_objects, Bullet, RotateBullet, SimpleBullet};
use crate::random_tools::simple_try;

const TRY_TIMES: u32 = 10;

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

#[derive(Clone)]
pub struct PlayerLocker {
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

    bullet_speed: f32,

    // status
    switch: bool, // on/off
}

impl CannonGeneratorInterface for PlayerLocker {
    fn generate(seed: u64, difficulty: f32) -> PlayerLocker {
        // difficulty expression
        // difficulty = fire_duration * count * (bullet_speed / fire_interval)
        // fire_freq = fd(cd * (0.2 - 1)) / cd(1 - 3) / fi(infer)
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        // cn * bs_ff^2
        let generated = simple_try(
            TRY_TIMES,
            |x| x[0] * x[1].powi(2),
            vec![(1., 7.), (0.5, 2.)],
            0.5,
            difficulty,
            rng.gen::<u64>(),
        );
        let (count, bs_ff) = (generated[0], generated[1]);
        let bs_ff_k = rng.gen_range(0.8, 1.2);
        let mut bullet_speed = (bs_ff * bs_ff_k).sqrt();
        let fire_interval = 0.3 * bullet_speed / bs_ff;
        bullet_speed *= 300.;
        let open_angle: f32 = rng.gen_range(-2f32, 1.2f32).exp();
        PlayerLocker {
            p: Point2f::new(),
            fire_interval,
            fire_cd: fire_interval,
            theta: 0., // uninitialized
            open_angle,
            count: count as u32,
            switch: true,
            bullet_speed,
            phase_timer: 0.,
        }
    }
}

impl PlayerLocker {
    fn update_theta(&mut self, player_p: Point2f, self_p: Point2f) {
        // r points to player
        let r = player_p - self_p;
        self.theta = r.y.atan2(r.x);
    }
}

impl CannonControllerInterface for PlayerLocker {
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
        loop {
            if self.fire_cd > dt {
                self.fire_cd -= dt;
                self.phase_timer += dt;
                break bullet_queue;
            }
            dt -= self.fire_cd;
            for x in 0..self.count {
                let normed_vec2f = Point2f::from_theta(
                    self.open_angle / (self.count + 1) as f32 * (x + 1) as f32 + self.theta
                        - self.open_angle / 2.,
                );
                bullet_queue.push_back(Box::new(SimpleBullet::new(
                    self.p + host_p,
                    normed_vec2f * self.bullet_speed,
                    Point2f::new(),
                    BULLET_RADIUS,
                    bullet_graphic_objects::WEDGE
                        .rotate(Mat2x2f::from_normed_vec2f(normed_vec2f)),
                )));
            }
            self.fire_cd = self.fire_interval;
        }
    }

    fn set_p(&mut self, p: Point2f) {
        self.p = p;
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

    bullet_speed: f32,

    // status
    switch: bool, // on/off
}

impl CannonGeneratorInterface for Rotor {
    fn generate(seed: u64, difficulty: f32) -> Rotor {
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        // difficulty = bullet_speed / fire_interval
        let generated = simple_try(
            TRY_TIMES,
            |x| x[0] / x[1],
            vec![(200., 400.), (0.2, 0.01)], // 1000-40000
            0.5,
            difficulty,
            rng.gen::<u64>(),
        );
        let (bullet_speed, fire_interval) = (generated[0], generated[1]);
        let omega: f32 = rng.gen_range(1., 6.);
        // let theta: f32 = rng.gen_range(0., 2. * std::f32::consts::PI);
        let theta: f32 = 0.;
        Rotor {
            p: Point2f::new(),
            fire_interval,
            fire_cd: fire_interval,
            theta,
            omega,
            bullet_speed,
            switch: true,
        }
    }
}

impl CannonControllerInterface for Rotor {
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
        self.theta += self.omega * dt;
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
                Point2f::from_polar(self.bullet_speed, self.theta),
                Point2f::new(),
                BULLET_RADIUS,
                rotate_matrix,
                bullet_graphic_objects::DIAMOND.clone(),
            )));
            self.fire_cd = self.fire_interval;
        }
    }

    fn set_p(&mut self, p: Point2f) {
        self.p = p;
    }
}

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
    rng: rand_pcg::Pcg64Mcg,

    bullet_speed: f32,

    // status
    switch: bool, // on/off
}

impl CannonGeneratorInterface for Shotgun {
    fn generate(seed: u64, difficulty: f32) -> Shotgun {
        // difficulty expression
        // difficulty = fire_duration * count * (bullet_speed / fire_interval)
        // fire_freq = fd(cd * (0.2 - 1)) / cd(1 - 3) / fi(infer)
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        // cn * bs_ff^2
        let generated = simple_try(
            TRY_TIMES,
            |x| x[0] * x[1].powi(2),
            vec![(1., 7.), (0.5, 2.)],
            0.5,
            difficulty,
            rng.gen::<u64>(),
        );
        let (count, bs_ff) = (generated[0], generated[1]);
        let bs_ff_k = rng.gen_range(0.8, 1.2);
        let mut bullet_speed = (bs_ff * bs_ff_k).sqrt();
        let fire_interval = 0.3 * bullet_speed / bs_ff;
        bullet_speed *= 300.;
        let open_angle: f32 = rng.gen_range(-1f32, 1.2f32).exp();
        Shotgun {
            p: Point2f::new(),
            fire_interval,
            fire_cd: fire_interval,
            theta: 0., // uninitialized
            open_angle,
            count: count as u32,
            rng: rand_pcg::Pcg64Mcg::seed_from_u64(rng.gen::<u64>()),
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
        loop {
            if self.fire_cd > dt {
                self.fire_cd -= dt;
                self.phase_timer += dt;
                break bullet_queue;
            }
            dt -= self.fire_cd;
            for _ in 0..self.count {
                let normed_vec2f = Point2f::from_theta(self.rng.gen_range(
                    self.theta - self.open_angle / 2.,
                    self.theta + self.open_angle / 2.,
                ));
                bullet_queue.push_back(Box::new(SimpleBullet::new(
                    self.p + host_p,
                    normed_vec2f * self.bullet_speed,
                    Point2f::new(),
                    BULLET_RADIUS,
                    bullet_graphic_objects::OCTAGON
                        .rotate(Mat2x2f::from_normed_vec2f(normed_vec2f)),
                )));
            }
            self.fire_cd = self.fire_interval;
        }
    }

    fn set_p(&mut self, p: Point2f) {
        self.p = p;
    }
}
