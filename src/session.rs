use crate::objects::Player;
use crate::graphic_object::{GraphicObject, GraphicObjects};

pub struct SessionGraphicObjectsIter<'a> {
    player_graphic_objects: &'a GraphicObjects,
    id: usize,
}

impl<'a> Iterator for SessionGraphicObjectsIter<'a> {
    type Item = &'a GraphicObject;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct Session {
    player: Player,
}

impl Session {
    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        self.player.proc_key(key_id, updown);
    }

    pub fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_graphic_objects: self.player
            id: 0,
        }
    }
}
