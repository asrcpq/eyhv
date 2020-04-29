use std::collections::VecDeque;

use crate::enemy::{Enemy, EnemyTickReturnOption};
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};
use crate::collision_pipe_interface::CollisionPipeInterface;

pub struct EnemyPool {
    enemies: VecDeque<Enemy>,
}

impl EnemyPool {
    pub fn new() -> EnemyPool {
        EnemyPool {
            enemies: VecDeque::new(),
        }
    }

    pub fn extend(&mut self, enemy_queue: VecDeque<Enemy>) {
        self.enemies.extend(enemy_queue);
    }

    pub fn tick(&mut self, dt: f32) {
        let len = self.enemies.len();
        for _ in 0..len {
            let mut enemy = self.enemies.pop_front().unwrap();
            // update pos
            match enemy.tick(dt) {
                EnemyTickReturnOption::Normal(bullet_queue) => {
                    // enemy bullet not implemented yet
                    self.enemies.push_back(enemy);
                },
                EnemyTickReturnOption::Destroyed |
                EnemyTickReturnOption::Removed => { }
            }
        }
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        let mut graphic_objects = GraphicObjects::new();
        for enemy in self.enemies.iter() {
            graphic_objects.extend(enemy.get_shifted_graphic_objects());
        }
        graphic_objects.into_iter()
    }
}

impl CollisionPipeInterface for EnemyPool {
    type Object = Enemy;

    fn push(&mut self, enemy: Enemy) {
        self.enemies.push_back(enemy);
    }
    fn pop(&mut self) -> Option<Enemy> {
        self.enemies.pop_front()
    }
    fn len(&self) -> usize {
        self.enemies.len()
    }
}
