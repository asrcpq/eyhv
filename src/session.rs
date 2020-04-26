use crate::graphic_object::GraphicObject;
use crate::objects::{MovingObjectGraphicsIter, Player};

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
    player: Player,
}

impl Session {
    pub fn new(resource_path: String) -> Session {
        Session {
            player: Player::new(resource_path + "graphic_objects/player.txt"),
        }
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        self.player.proc_key(key_id, updown);
    }

    pub fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_iter: self.player.graphics_iter(),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.player.update_p(dt)
    }
}
