use crate::objects::Player;
use crate::graphic_object::{GraphicObject, GraphicObjects};

pub struct Session {
    player: Player,
}

impl Session {
    pub fn new(resource_path: &str) -> Session {
        Session {
            player: Player::new(resource_path),
        }
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        self.player.proc_key(key_id, updown);
    }

    //pub fn graphic_object_iter(&self) -> impl Iterator<Item = GraphicObject> {
    //    self.player.graphics_iter()
    //}
}
