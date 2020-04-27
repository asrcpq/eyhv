use crate::algebra::{Point2f, Rect2f};
use crate::graphic_object::GraphicObjects;
use crate::moving_object::{MovingObject, MovingObjectGraphicsIter};

pub struct Player {
    // Dynamic
    p: Point2f,
    dp: Point2f,

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
            graphic_objects: GraphicObjects::from_strs(
                vec!["l 0.3 1.0 1.0 1.0 -10 8 0 -10 10 8 3 4 -3 4 -10 8"]
            ),
            // these should be written in a config file
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
        const SQRT_1_2: f32 = 0.7071067811865476;
        if dp.x != 0. && dp.y != 0. {
            dp *= SQRT_1_2;
        }
        dp *= self.speed;
        self.dp = dp;
    }

    pub fn update_p(&mut self, dt_scaled: f32, directions: &[bool; 4], window_size: Rect2f) {
        self.set_dp(directions);
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
