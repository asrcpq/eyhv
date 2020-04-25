// Moving objects

use crate::algebra::Point2f;
use crate::graphic_object::GraphicObject;
use crate::key_state::KeyState;

struct MovingObject {
    // Dynamic
    p: Point2f,
    dp: Point2f,

    // Static
    graphic_objects: Vec<GraphicObject>,
}

impl MovingObject {
    pub fn set_dp(&mut self, dp: Point2f) {
        self.dp = dp;
    }

    pub fn update_p(&mut self) {
        self.p += self.dp;
    }
}

pub struct Player {
    object: MovingObject,

    // control
    key_state: KeyState,

    // params
    speed: f32, // per second
}

impl Player {
    // proc_key is executed when valid key is pressed
    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        self.key_state.proc_key(key_id, updown);
    }

    // set_dp is executed before frame update
    pub fn set_dp(&mut self) {
        let mut dp = Point2f::new();
        for (key_id, updown) in self.key_state.directions.iter().enumerate() {
            if *updown {
                match key_id {
                    0 => dp.x -= 1.,
                    2 => dp.x += 1.,
                    1 => dp.y -= 1.,
                    3 => dp.y += 1.,
                    _ => panic!("unexpected keycode"),
                }
            }
            
            //diagonal correction
            const sqrt_1_2: f32 = 0.7071067811865476;
            if dp.x != 0. && dp.y != 0. {
                dp *= sqrt_1_2;
            }
        }
    }
}
