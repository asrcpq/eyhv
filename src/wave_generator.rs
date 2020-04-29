use std::collections::VecDeque;
use std::enemy;
use std::enemy::Enemy;

pub struct WaveGenerator {
    wave_cd: f32,
    wave_interval: f32,
}

impl WaveGenerator {
    pub fn new() -> WaveGenerator {
        WaveGenerator {
            wave_cd: 2.,
            wave_interval: 10.,
        }
    }

    // dummy
    pub fn tick(dt: f32) -> VecDeque<Enemy> {
        let mut enemy_queue: VecDeque<Enemy> = VecDeque::new();
        while dt > 0 {
            if wave_cd > dt {
                wave_cd -= dt;
                dt = 0;
            } else {
                wave_cd = wave_interval;
                enemy_queue.push_back(Enemy::dummy(enemy::DummyEnemy::new()));
            }
        }
        enemy_queue
    }
}
