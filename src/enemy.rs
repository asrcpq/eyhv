use lazy_static::lazy_static;

use crate::algebra::{Point2f, Mat2x2f};
use crate::graphic_object::GraphicObjects;

// This struct is static, created by Session::new() only once
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

