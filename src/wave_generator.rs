use crate::enemy;
use crate::enemy::Enemy;
use std::collections::VecDeque;

pub struct WaveGenerator {
    wave_cd: f32,
    wave_interval: f32,
}

impl WaveGenerator {
    pub fn new() -> WaveGenerator {
        WaveGenerator {
            wave_cd: 1.,
            wave_interval: 6.,
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
                enemy_queue.push_back(Enemy::Dummy(enemy::DummyEnemy::new()));
            }
        }
        enemy_queue
    }
}
