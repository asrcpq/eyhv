use std::collections::VecDeque;

use rand::SeedableRng;
use rand::Rng;
use rand_pcg;

use crate::algebra::Point2f;
use crate::bullet;
use crate::graphic_object::GraphicObjects;

pub trait CannonControllerInterface {
    // once a cannon is turned off, it immediately resets the state of itself
    // static implementation
    fn switch(&mut self, switch: bool);
    // this is called fire_tick as there might be other tick functions
    // like PlayerLocker's update_theta
    fn fire_tick(&mut self, host_p: Point2f, dt: f32) -> VecDeque<bullet::Bullet>;
}

pub trait CannonGeneratorInterface {
    fn generate(p: Point2f, seed: u64, difficulty: f32) -> Self;
}

pub struct PlayerLocker {
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
    open_angle: f32,
    count: u32,

    // status
    switch: bool, // on/off
}

impl CannonGeneratorInterface for PlayerLocker {
    fn generate(p: Point2f, seed: u64, difficulty: f32) -> PlayerLocker {
        // difficulty expression
        // difficulty = fire_freq * count * bullet_speed
        const MINIMAL_ALLOC: f32 = 0.2;
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        unimplemented!()
    }
    // call update_theta after creating
    // pub fn new(p: Point2f, fd: f32, cd: f32, fi: f32, oa: f32, cn: u32, sw: bool) -> PlayerLocker {
    //     PlayerLocker {
    //         p: p,
    //         fire_duration: fd,
    //         cycle_duration: cd,
    //         fire_interval: fi,
    //         fire_cd: fi,
    //         theta: 0., // not initialized
    //         open_angle: oa,
    //         count: cn,
    //         switch: sw,
    //         phase_timer: 0.,
    //     }
    // }

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
        if self.switch {
            if !switch {
                self.switch = false;
                self.phase_timer = 0.;
                self.fire_cd = self.fire_interval;
            }
        } else {
            if switch {
                self.switch = true;
            }
        }
    }

    fn fire_tick(&mut self, host_p: Point2f, mut dt: f32) -> VecDeque<bullet::Bullet> {
        let mut bullet_queue = VecDeque::new();
        const BULLET_SPEED: f32 = 100.;
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
                    break bullet_queue;
                } else {
                    dt -= self.cycle_duration - self.phase_timer;
                    self.phase_timer = 0.;
                    self.fire_cd = self.fire_interval;
                }
            }
            while self.phase_timer < self.fire_duration {
                if self.fire_cd > dt {
                    self.fire_cd -= dt;
                    self.phase_timer += dt;
                    break'cycle bullet_queue;
                }
                dt -= self.fire_cd;
                bullet_queue.push_back(bullet::Bullet::Simple(bullet::SimpleBullet::new(
                    self.p + host_p,
                    Point2f::from_floats(0., -BULLET_SPEED),
                    Point2f::new(),
                    BULLET_RADIUS,
                    bullet::BULLET_GRAPHIC_OBJECTS.wedge.clone(),
                )));
                self.fire_cd = self.fire_interval;
            }
        }
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
            p: p,
            fire_interval: fi,
            // player should not benefit from a rapid fire controller
            fire_cd: fi,
            v: v,
            switch: sw,
        }
    }
}

impl CannonControllerInterface for SimpleCannon {
    fn switch(&mut self, switch: bool) {
        if self.switch {
            if !switch {
                self.switch = false;
                self.fire_cd = self.fire_interval;
            }
        } else {
            if switch {
                self.switch = true;
            }
        }
    }

    fn fire_tick(&mut self, host_p: Point2f, mut dt: f32) -> VecDeque<bullet::Bullet> {
        const BULLET_SPEED: f32 = 2500.;
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
                bullet_queue.push_back(bullet::Bullet::Simple(bullet::SimpleBullet::new(
                    self.p + host_p,
                    self.v,
                    Point2f::new(),
                    BULLET_RADIUS,
                    bullet::BULLET_GRAPHIC_OBJECTS.rectangle.clone(),
                )));
            }
        }
    }
}
