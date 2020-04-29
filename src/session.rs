use crate::algebra::Rect2f;
use crate::graphic_object::{GraphicObject, GraphicObjectsIntoIter};
use crate::key_state::KeyState;
use crate::player::Player;
use crate::time_manager::TimeManager;
use crate::bullet_pool::BulletPool;

pub struct SessionGraphicObjectsIter {
    player_iter: GraphicObjectsIntoIter,
    player_bullet_iter: GraphicObjectsIntoIter,
}

impl Iterator for SessionGraphicObjectsIter {
    type Item = GraphicObject;

    fn next(&mut self) -> Option<GraphicObject> {
        match self.player_iter.next() {
            None => {},
            option => return option,
        }
        match self.player_bullet_iter.next() {
            None => {},
            option => return option,
        }
        None
    }
}

pub struct Session {
    player: Player,

    player_bullet_pool: BulletPool,

    // control
    key_state: KeyState,

    time_manager: TimeManager,
}

impl Session {
    pub fn new() -> Session {
        Session {
            player: Player::new(),
            player_bullet_pool: BulletPool::new(),
            key_state: KeyState::new(),
            time_manager: TimeManager::new(),
        }
    }

    pub fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_iter: self.player.graphic_objects_iter(),
            player_bullet_iter: self.player_bullet_pool.graphic_objects_iter(),
        }
    }

    pub fn tick(&mut self, mut dt: f32) {
        dt *= self.time_manager.update_and_get_dt_scaler(dt);
        self.player_bullet_pool.tick(dt);
        self.player_bullet_pool.extend(self.player.tick(
            dt,
            &self.key_state.directions
        ));
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        if key_id == 4 {
            self.time_manager.set_state(updown);
        } else if key_id == 5 {
            self.player.switch_cannons(updown);
        } else {
            self.key_state.proc_key(key_id, updown);
        }
    }
}
