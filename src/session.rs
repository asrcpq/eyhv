use crate::graphic_object::GraphicObject;
use crate::moving_object::{MovingObjectGraphicsIter, MovingObject};
use crate::player::Player;
use crate::algebra::Rect2f;

pub struct SessionGraphicObjectsIter<'a> {
    player_iter: MovingObjectGraphicsIter<'a>,
}

impl<'a> Iterator for SessionGraphicObjectsIter<'a> {
    type Item = GraphicObject;

    fn next(&mut self) -> Option<GraphicObject> {
        self.player_iter.next()
    }
}

pub struct Session {
    window_size: Rect2f,
    player: Player,
}

impl Session {
    pub fn new(window_size: Rect2f, resource_root: String) -> Session {
        Session {
            window_size: window_size,
            player: Player::new(resource_root + "graphic_objects/player.txt"),
        }
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        self.player.proc_key(key_id, updown);
    }

    pub fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_iter: self.player.moving_object_graphics_iter(),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.player.update_p(dt, self.window_size)
    }
}
