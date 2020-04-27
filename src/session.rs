use crate::algebra::Rect2f;
use crate::graphic_object::GraphicObject;
use crate::key_state::KeyState;
use crate::moving_object::{MovingObject, MovingObjectGraphicsIter};
use crate::player::Player;

pub struct SessionGraphicObjectsIter<'a> {
    player_iter: MovingObjectGraphicsIter<'a>,
}

impl<'a> Iterator for SessionGraphicObjectsIter<'a> {
    type Item = GraphicObject;

    fn next(&mut self) -> Option<GraphicObject> {
        self.player_iter.next()
    }
}

struct TimeManager {
    dt_scaler: f32,
    slowdown: bool,
    shifting: bool,
    // absolute value of scaler per second, not percentage of difference
    shift_rate: f32,
    dt_scaler_normal: f32,
    dt_scaler_slow: f32,
}

impl TimeManager {
    pub fn new() -> TimeManager {
        TimeManager {
            dt_scaler: 1.,
            slowdown: false,
            shifting: false,
            shift_rate: 0.5,
            dt_scaler_normal: 1.,
            dt_scaler_slow: 0.5,
        }
    }

    pub fn set_state(&mut self, slowdown: bool) {
        self.slowdown = slowdown;
        if slowdown && self.dt_scaler > self.dt_scaler_slow ||
            !slowdown && self.dt_scaler < self.dt_scaler_normal {
            self.shifting = true
        }
    }

    fn update_scaler(&mut self) {
        if self.shifting {
            if self.slowdown {
                self.dt_scaler -= self.shift_rate;
                if self.dt_scaler < self.dt_scaler_slow {
                    self.dt_scaler = self.dt_scaler_slow;
                }
            } else {
                self.dt_scaler += self.shift_rate;
                if self.dt_scaler > self.dt_scaler_normal {
                    self.dt_scaler = self.dt_scaler_normal;
                }
            }
        }
    }

    pub fn update_and_get_dt_scaler(&mut self) -> f32 {
        self.update_scaler();
        self.dt_scaler
    }
}

pub struct Session {
    window_size: Rect2f,
    player: Player,

    // control
    key_state: KeyState,

    time_manager: TimeManager,
}

impl Session {
    pub fn new(window_size: Rect2f, resource_root: String) -> Session {
        Session {
            window_size: window_size,
            player: Player::new(resource_root + "graphic_objects/player.txt"),
            key_state: KeyState::new(),
            time_manager: TimeManager::new(),
        }
    }

    pub fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_iter: self.player.moving_object_graphics_iter(),
        }
    }

    pub fn tick(&mut self, mut dt: f32) {
        dt *= self.time_manager.update_and_get_dt_scaler();
        self.player.update_p(dt, &self.key_state, self.window_size)
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        if key_id == 4 {
            self.time_manager.set_state(updown);
        } else {
            self.key_state.proc_key(key_id, updown);
        }
    }
}
