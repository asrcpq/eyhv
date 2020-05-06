use std::collections::VecDeque;

use crate::algebra::{Point2f, Mat2x2f};
use crate::bullet::{bullet_graphic_objects, Bullet, SimpleBullet};

// SimpleCannon fires bullets with the same and constant speed
// in the same direction at regular intervals
// It is designed for Player
pub struct SimpleCannon {
    // relative to moving object
    p: Point2f,

    fire_interval: f32,
    fire_cd: f32,

    side_cannon_angle: u32,

    // for player, -90 deg is facing forward
    vy: f32,

    pub switch: bool,
}

impl SimpleCannon {
    pub fn new(p: Point2f, vy: f32, fi: f32, sw: bool) -> SimpleCannon {
        SimpleCannon {
            p,
            fire_interval: fi,
            // player should not benefit from a rapid fire controller
            fire_cd: fi,
            side_cannon_angle: 1,
            vy,
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

    pub fn tick(&mut self, host_p: Point2f, mut dt: f32, side_cannon_off: bool) -> VecDeque<Box<dyn Bullet>> {
        const BULLET_RADIUS: f32 = 3.;
        const SIDE_CANNON_CD: u32 = 5;
        const SIDE_CANNON_RESET: u32 = 10;
        let angle_k: f32 = 5f32.to_radians();
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
                for left_mid_right in if side_cannon_off ||
                    self.side_cannon_angle > SIDE_CANNON_CD {
                    0..1
                } else {
                    -1..2
                } {
                    bullet_queue.push_back(Box::new(SimpleBullet::new(
                        self.p + host_p,
                        Point2f::from_polar(
                            self.vy,
                            std::f32::consts::FRAC_PI_2 +
                                angle_k * self.side_cannon_angle as f32 * left_mid_right as f32,
                        ),
                        Point2f::new(),
                        dt,
                        BULLET_RADIUS,
                        bullet_graphic_objects::RECTANGLE_PLAYER.rotate(Mat2x2f::from_theta(
                            angle_k * self.side_cannon_angle as f32 * left_mid_right as f32,
                        )),
                    )));
                }
                // mod before add so angles will move +1
                self.side_cannon_angle %= SIDE_CANNON_RESET;
                self.side_cannon_angle += 1;
            }
        }
    }
}
