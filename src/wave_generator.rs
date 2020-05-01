use std::collections::VecDeque;

use crate::cannon::CannonGeneratorInterface;
use crate::enemy::Enemy;

#[derive(Clone)]
pub struct WaveScheme {
    Enemy:
}

pub struct WaveGenerator {
    test_enemy: Enemy,
    wave_cd: f32,
    wave_interval: f32,
}

impl WaveGenerator {
    pub fn new() -> WaveGenerator {
        WaveGenerator {
            test_enemy: Enemy::new_small(),
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
                enemy_queue.push_back(self.test_enemy.clone());
            }
        }
        enemy_queue
    }
}
