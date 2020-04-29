use lazy_static::lazy_static;

use crate::graphic_object::GraphicObjects;
use crate::enemy_path;

// This struct is static, created by Session::new() only once
#[derive(Clone)]
pub struct EnemyGraphicObjects {
    pub dummy: GraphicObjects,
}

lazy_static! {
    pub static ref ENEMY_GRAPHIC_OBJECTS: EnemyGraphicObjects = {
        EnemyGraphicObjects {
            dummy: GraphicObjects::from_strs(
                vec!["l 1 1 1 1 -10 -10 -10 10 10 10 10 -10 -10 -10"]
            ),
        }
    };
}

pub enum Enemy {
    Dummy(DummyEnemy),
}

pub struct DummyEnemy {
    path: enemy_path::EnemyPath,

    graphic_objects: GraphicObjects,
}

impl DummyEnemy {
    pub fn new() -> DummyEnemy {
        DummyEnemy {
            path: enemy_path::EnemyPath::Straight(
                enemy_path::StraightDown::new(200., 100.)
            ),
            graphic_objects: ENEMY_GRAPHIC_OBJECTS.dummy.clone(),
        }
    }

    pub fn get_shifted_graphic_objects() {
        
    }
}
