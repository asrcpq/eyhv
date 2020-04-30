use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::bullet::Bullet;
use crate::collision::CollisionPipeInterface;
use crate::enemy::{Enemy, EnemyTickReturnOption};
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};

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

    pub fn tick(&mut self, dt: f32, player_p: Point2f) -> VecDeque<Bullet> {
        let len = self.enemies.len();
        let mut bullet_queue_return = VecDeque::new();
        for _ in 0..len {
            let mut enemy = self.enemies.pop_front().unwrap();
            // update pos
            match enemy.tick(dt, player_p) {
                EnemyTickReturnOption::Normal(bullet_queue) => {
                    bullet_queue_return.extend(bullet_queue);
                    self.enemies.push_back(enemy);
                }
                EnemyTickReturnOption::Destroyed | EnemyTickReturnOption::Removed => {}
            }
        }

        bullet_queue_return
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
