use crate::algebra::{Point2f, Rect2f};
use crate::graphic_object::{GraphicObject, GraphicObjects};
use crate::key_state::KeyState;
use crate::moving_object::{MovingObject, MovingObjectGraphicsIter};

pub struct Player {
    // Dynamic
    p: Point2f,
    dp: Point2f,

    // Static
    graphic_objects: GraphicObjects,

    // params
    speed_normal: f32, // per second
    speed_slow: f32,
}

impl Player {
    pub fn new(resource_path: String) -> Player {
        Player {
            p: Point2f::from_floats(50.0, 50.0),
            dp: Point2f::new(),
            graphic_objects: GraphicObjects::from_path(resource_path),
            // these should be written in a config file
            speed_normal: 600.0,
            speed_slow: 350.0,
        }
    }

    // set_dp is executed before frame update
    pub fn set_dp(&mut self, key_state: &KeyState) {
        let mut dp = Point2f::new();
        for (key_id, updown) in key_state.directions.iter().enumerate() {
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
        const SQRT_1_2: f32 = 0.7071067811865476;
        if dp.x != 0. && dp.y != 0. {
            dp *= SQRT_1_2;
        }
        if key_state.slowdown {
            dp *= self.speed_slow;
        } else {
            dp *= self.speed_normal;
        }
        self.dp = dp;
    }

    pub fn update_p(&mut self, dt_scaled: f32, key_state: &KeyState, window_size: Rect2f) {
        self.set_dp(key_state);
        self.p += self.dp * dt_scaled;
        self.p = window_size.nearest(self.p);
    }
}

impl MovingObject for Player {
    fn get_p(&self) -> Point2f {
        self.p
    }

    fn moving_object_graphics_iter(&self) -> MovingObjectGraphicsIter {
        MovingObjectGraphicsIter::new(self.p, &self.graphic_objects)
    }
}
