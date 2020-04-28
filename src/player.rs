use std::collections::VecDeque;

use crate::algebra::{Point2f, Rect2f};
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};
use crate::cannon::{SimpleCannon, CannonControllerInterface};
use crate::bullet::SimpleBullet;

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
        Player {
            p: Point2f::from_floats(50.0, 50.0),
            dp: Point2f::new(),
            cannons: vec![
                SimpleCannon::new(
                    Point2f::from_floats(-2., 0.),
                    -std::f32::consts::FRAC_PI_2,
                    false,
                ),
                SimpleCannon::new(
                    Point2f::from_floats(2., 0.),
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

    pub fn tick(&mut self, dt: f32, directions: &[bool; 4], window_size: Rect2f) -> VecDeque<SimpleBullet> {
        self.set_dp(directions);
        self.p += self.dp * dt;
        self.p = window_size.nearest(self.p);

        let mut bullet_queue = VecDeque::new();
        for cannon in self.cannons.iter_mut() {
            bullet_queue.extend(cannon.fire_tick(self.p, dt));
        }
        bullet_queue
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        GraphicObjectsIntoIter::new(self.graphic_objects.shift(self.p))
    }
}
