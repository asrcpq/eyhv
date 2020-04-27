use std::collections::VecDeque;

use crate::algebra::Point2f;

pub enum CannonTypes {
    PlayerLocker(PlayerLocker),
}

pub trait CannonControllerInterface {
    // once a cannon is turned off, it immediately resets the state of itself
    fn switch(&mut self, switch: bool);

    fn tick(&mut self, dt: f32) -> BulletQueue;
}

pub struct PlayerLocker {
    // static parameters

    // relative to moving object
    p: Point2f,

    // Durations, phase = fire + cd
    fire_duration: f32,
    phase_duration: f32,

    // bullet shooted during fire phase
    fire_interval: f32,

    // angle and bullet number
    // bullets are uniformly distributed on opening angle
    // and are shooted together
    angle: f32,
    count: u32,

    // status

    switch: bool, // on/off

    // phase_timer takes value from 0-phase_duration, and reset
    phase_timer: f32,
}

impl PlayerLocker {
    pub fn new(p: Point2f, fd: f32, pd: f32, fi: f32, ag: f32, cn: u32, sw: bool) -> PlayerLocker{
        PlayerLocker {
            p: p,
            fire_duration: fd,
            phase_duration: pd,
            fire_interval: fi,
            angle: ag,
            count: cn,
            switch: sw,
            phase_timer: 0.,
        }
    }
}

impl Cannon for PlayerLocker {
    fn switch(&mut self, switch: bool) {
        if self.switch {
            if !switch {
                self.switch = false;
                self.phase_timer = 0;
            }
        } else {
            if switch {
                self.switch = true;
            }
        }
    }

    fn tick(&mut self, dt: f32) -> VecDeque<Bullet> {
        if self.phase_timer > self.fire_duration {
        }
    }
}
