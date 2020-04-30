use std::collections::VecDeque;

use crate::cannon::CannonGeneratorInterface;
use crate::enemy::Enemy;

enum WaveScheme {}

pub struct WaveGenerator {
    dummy_enemy: Enemy,
    wave_cd: f32,
    wave_interval: f32,
}

impl WaveGenerator {
    pub fn new() -> WaveGenerator {
        WaveGenerator {
            dummy_enemy: Enemy::new_dummy(),
            wave_cd: 1.,
            wave_interval: 7.,
        }
    }

    // dummy
    pub fn tick(&mut self, mut dt: f32) -> VecDeque<Enemy> {
        let mut enemy_queue: VecDeque<Enemy> = VecDeque::new();
        while dt > 0. {
            if self.wave_cd > dt {
                self.wave_cd -= dt;
                dt = 0.;
            } else {
                self.wave_cd = self.wave_interval;
                enemy_queue.push_back(self.dummy_enemy.clone());
            }
        }
        enemy_queue
    }
}
