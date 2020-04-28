use std::collections::VecDeque;

use crate::bullet::Bullet;
use crate::algebra::Rect2f;
use crate::moving_object::{MovingObject, MovingObjectGraphicsIter};
use crate::graphic_object::GraphicObjects;

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

    pub fn tick(&mut self, window_size: Rect2f, dt: f32) {
        let len = self.bullets.len();
        for _ in 0..len {
            let mut bullet = self.bullets.pop_front().unwrap();
            // update pos
            bullet.tick(dt);

            // check pos
            if !window_size.contain(bullet.get_p()) {
                continue
            }

            self.bullets.push_back(bullet);
        }
    }

    pub fn moving_object_graphics_iter(&self) -> MovingObjectGraphicsIter {
        
        for bullet in self.bullets.iter() {
            bullet.moving_object_graphics_iter().shift(bullet.get_p())
        }
    }
}
