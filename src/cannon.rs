use std::collections::VecDeque;

use crate::algebra::Point2f;

pub enum CannonTypes {
    PlayerLocker(PlayerLocker),
}

pub trait CannonControllerInterface {
    // once a cannon is turned off, it immediately resets the state of itself
    // static implementation
    fn switch(&mut self, switch: bool);

    // static implementation
    fn get_absolute_p(&self, unit_p: Point2f) -> Point2f;

    // this is called fire_tick as there might be other tick functions
    // like PlayerLocker's update_theta
    fn fire_tick(&mut self, dt: f32) -> BulletQueue;
}

pub struct PlayerLocker {
    // static parameters

    // relative to moving object
    p: Point2f,

    // Durations, phase = fire + cd
    fire_duration: f32,
    cycle_duration: f32,

    // bullet shooted during fire phase
    fire_interval: f32,

    // timer between intervals
    fire_countdown: f32,

    // direction, opening angle and bullet number
    // bullets are uniformly distributed on opening angle
    // and are shooted together
    theta: f32,
    open_angle: f32,
    count: u32,

    // status

    switch: bool, // on/off

    // phase_timer takes value from 0-cycle_duration, and reset
    phase_timer: f32,
}

impl PlayerLocker {
    // call update_theta after creating
    pub fn new(p: Point2f, fd: f32, cd: f32, fi: f32, oa: f32, cn: u32, sw: bool) -> PlayerLocker{
        PlayerLocker {
            p: p,
            fire_duration: fd,
            cycle_duration: cd,
            fire_interval: fi,
            fire_countdown: 0, //fire immediately
            theta: 0, // not initialized
            angle: oa,
            count: cn,
            switch: sw,
            phase_timer: 0.,
        }
    }

    fn update_theta() {
    }
}

impl Cannon for PlayerLocker {
    fn switch(&mut self, switch: bool) {
        if self.switch {
            if !switch {
                self.switch = false;
                self.phase_timer = 0;
                self.fire_cd = self.fire_interval;
            }
        } else {
            if switch {
                self.switch = true;
            }
        }
    }

    fn get_absolute_p(&self, unit_p: Point2f) -> Point2f {
        unit_p + self.p
    }

    fn tick(&mut self, mut dt: f32) -> VecDeque<Bullet> {
        if self.phase_timer > self.fire_duration {
            self.phase_timer += dt;
            if self.phase_timer < self.cycle_duration {
                VecDequeue::new()
            } else {
                self.phase_timer -= self.cycle_duration;
            }
        } else {
            
        }
    }
}
