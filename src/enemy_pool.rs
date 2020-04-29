use std::collections::VecDeque;

use crate::enemy::Enemy;
use crate::graphic_objects::{GraphicObjects, GraphicObjectsIntoIter};

pub struct EnemyPool {
    enemies: VecDeque<Enemy>,
}

impl EnemyPool {
    pub fn new() -> EnemyPool {
        enemys:: VecDeque::new(),
    }

    pub fn extend(&mut self, enemy_queue: VecDeque<Enemy>) {
        self.enemys.extend(enemy_queue);
    }

    pub fn tick(&mut self, dt: f32) {
        let len = self.enemys.len();
        for _ in 0..len {
            let mut enemy = self.enemys.pop_front().unwrap();
            // update pos
            enemy.tick(dt);

            //
        }
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        let mut graphic_objects = GraphicObjects::new();
        for enemy in self.enemies {
            graphic_objects.extend(enemy.get_shifted_graphic_objects());
        }
        graphic_objects
    }
}
