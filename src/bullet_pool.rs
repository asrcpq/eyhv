use std::collections::VecDeque;

use crate::bullet::Bullet;
use crate::algebra::Rect2f;
use crate::moving_object::MovingObject;

pub struct BulletPool {
    bullets: VecDeque<Bullet>
}

impl BulletPool {
    pub fn new() -> BulletPool {
        BulletPool {
            bullets: VecDeque::new(),
        }
    }

    pub fn append(&mut self, mut bullet_queue: VecDeque<Bullet>) {
        self.bullets.append(&mut bullet_queue);
    }

    pub fn tick(&mut self, window_size: Rect2f, dt: f32) {
        // This implementation fails as compiler rejects to move self.bullets
        //let mut old_bullets = self.bullets;
        //self.bullets = VecDeque::new();
        //for mut bullet in old_bullets.drain(..) {
        //    // update pos
        //    bullet.tick(dt);

        //    // check pos
        //    if !window_size.contain(bullet.get_p()) {
        //        continue
        //    }

        //    self.bullets.push_back(bullet);
        //}

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
}
