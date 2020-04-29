use std::collections::VecDeque;

use crate::algebra::Rect2f;
use crate::bullet::Bullet;
use crate::collision_pipe_interface::CollisionPipeInterface;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};
use crate::window_rect::WINDOW_RECT;

pub struct BulletPool {
    bullets: VecDeque<Bullet>,
}

impl BulletPool {
    pub fn new() -> BulletPool {
        BulletPool {
            bullets: VecDeque::new(),
        }
    }

    pub fn extend(&mut self, bullet_queue: VecDeque<Bullet>) {
        self.bullets.extend(bullet_queue);
    }

    pub fn tick(&mut self, dt: f32) {
        let len = self.bullets.len();
        for _ in 0..len {
            let mut bullet = self.bullets.pop_front().unwrap();
            // update pos
            bullet.tick(dt);

            // check pos
            if let Some(p) = bullet.get_p() {
                if !WINDOW_RECT.contain(p) {
                    continue;
                }
            }

            self.bullets.push_back(bullet);
        }
    }

    // only for collision
    pub fn pop(&mut self) -> Option<Bullet> {
        self.bullets.pop_front()
    }

    // only for collision
    pub fn push(&mut self, bullet: Bullet) {
        self.bullets.push_back(bullet)
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        let mut graphic_objects = GraphicObjects::new();
        for bullet in self.bullets.iter() {
            graphic_objects.extend(bullet.get_shifted_graphic_objects());
        }
        graphic_objects.into_iter()
    }
}

impl CollisionPipeInterface for BulletPool {
    type Object = Bullet;

    fn push(&mut self, enemy: Bullet) {
        self.bullets.push_back(enemy);
    }
    fn pop(&mut self) -> Option<Bullet> {
        self.bullets.pop_front()
    }
    fn len(&self) -> usize {
        self.bullets.len()
    }
}
