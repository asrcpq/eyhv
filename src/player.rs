use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};
use crate::cannon::{SimpleCannon, CannonControllerInterface};
use crate::bullet::{Bullet, SimpleBullet};
use crate::window_rect::WINDOW_RECT;

pub struct Player {
    // Dynamic
    p: Point2f,
    dp: Point2f,

    cannons: Vec<SimpleCannon>,

    // Static
    graphic_objects: GraphicObjects,

    // params
    speed: f32, // per second
}

impl Player {
    pub fn new() -> Player {
        let window_size = WINDOW_RECT.get_size();
        Player {
            p: Point2f::from_floats(
                window_size.x / 2.,
                window_size.y - 50.,
            ),
            dp: Point2f::new(),
            cannons: vec![
                SimpleCannon::new(
                    Point2f::from_floats(-3., -5.),
                    -std::f32::consts::FRAC_PI_2,
                    false,
                ),
                SimpleCannon::new(
                    Point2f::from_floats(3., -5.),
                    -std::f32::consts::FRAC_PI_2,
                    false,
                ),
            ],
            graphic_objects: GraphicObjects::from_strs(vec![
                "l 0.3 1.0 1.0 1.0 -10 8 0 -10 10 8 3 4 -3 4 -10 8",
            ]),
            speed: 600.0,
        }
    }

    // set_dp is executed before frame update
    pub fn set_dp(&mut self, directions: &[bool; 4]) {
        let mut dp = Point2f::new();
        for (key_id, updown) in directions.iter().enumerate() {
            if *updown {
                match key_id {
                    0 => dp.x -= 1.,
                    2 => dp.x += 1.,
                    1 => dp.y -= 1.,
                    3 => dp.y += 1.,
                    _ => panic!("unexpected keycode"),
                }
            }
        }

        //diagonal correction
        if dp.x != 0. && dp.y != 0. {
            dp *= std::f32::consts::FRAC_1_SQRT_2;
        }
        dp *= self.speed;
        self.dp = dp;
    }

    pub fn tick(&mut self, dt: f32, directions: &[bool; 4]) -> VecDeque<Bullet> {
        self.set_dp(directions);
        self.p += self.dp * dt;
        self.p = WINDOW_RECT.nearest(self.p);

        let mut bullet_queue = VecDeque::new();
        for cannon in self.cannons.iter_mut() {
            bullet_queue.extend(cannon.fire_tick(self.p, dt));
        }
        bullet_queue
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        self.graphic_objects.shift(self.p).into_iter()
    }

    pub fn switch_cannons(&mut self, switch: bool) {
        for cannon in self.cannons.iter_mut() {
            cannon.switch(switch);
        }
    }
}
