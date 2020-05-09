use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::bullet::Bullet;
use crate::collision::CollisionPipeInterface;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};
use crate::window_rect::WINDOW_RECT;

use lazy_static::lazy_static;

pub struct BulletPool {
    bullets: VecDeque<Box<dyn Bullet>>,
}

impl BulletPool {
    pub fn new() -> BulletPool {
        BulletPool {
            bullets: VecDeque::new(),
        }
    }

    pub fn extend(&mut self, bullet_queue: VecDeque<Box<dyn Bullet>>) {
        self.bullets.extend(bullet_queue);
    }

    pub fn tick(&mut self, dt: f32) {
        let len = self.bullets.len();
        for _ in 0..len {
            let mut bullet = self.bullets.pop_front().unwrap();
            // check pos before update
            // so the bullet which just moves out of screen
            // will be catched in collision test
            if !WINDOW_RECT.contain(bullet.get_p()) {
                continue;
            }
            bullet.tick(dt);

            self.bullets.push_back(bullet);
        }
    }

    // only for collision
    pub fn pop(&mut self) -> Option<Box<dyn Bullet>> {
        self.bullets.pop_front()
    }

    // only for collision
    pub fn push(&mut self, bullet: Box<dyn Bullet>) {
        self.bullets.push_back(bullet)
    }

    pub fn graphic_objects_iter(&self, show_pos: bool) -> GraphicObjectsIntoIter {
        lazy_static! {
            static ref MARK: GraphicObjects = GraphicObjects::from_strs(vec![
                "l 1 1 1 1 -1 -1 1 1",
                "l 1 1 1 1 1 -1 -1 1",
            ]).zoom(2.);
        }
        let mut graphic_objects: GraphicObjects = Default::default();
        for bullet in self.bullets.iter() {
            graphic_objects.extend(bullet.get_shifted_graphic_objects());
            if show_pos {
                graphic_objects.extend(MARK.shift(bullet.get_p()))
            }
        }
        graphic_objects.into_iter()
    }
}

impl CollisionPipeInterface for BulletPool {
    type Object = Box<dyn Bullet>;

    fn push(&mut self, enemy: Box<dyn Bullet>) {
        self.bullets.push_back(enemy);
    }
    fn pop(&mut self) -> Option<Box<dyn Bullet>> {
        self.bullets.pop_front()
    }
    fn len(&self) -> usize {
        self.bullets.len()
    }
}
