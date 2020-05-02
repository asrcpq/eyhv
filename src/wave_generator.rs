use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::cannon;
use crate::enemy::Enemy;
use crate::enemy_path::enemy_paths;

mod wave_scheme_prototype {
    use std::collections::VecDeque;

    use crate::enemy::Enemy;
    use crate::enemy::enemy_prototype;
    use crate::enemy_path::{EnemyPath, enemy_paths};
    use crate::cannon;
    use crate::cannon::{CannonControllerInterface};
    use crate::graphic_object::GraphicObjects;
    use super::CompiledWave;
    
    use lazy_static::lazy_static;
    use rand::Rng;
    use rand::SeedableRng;
    use rand::seq::SliceRandom;
    use rand_pcg;

    #[derive(Clone)]
    pub struct WaveSchemePrototype {
        pub enemies: Vec<(enemy_prototype::EnemyPrototype, Vec<(EnemyPath, Vec<f32>)>)>,
    }

    lazy_static! {
        static ref LEFT_RIGHT_CHAINS: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(enemy_prototype::SMALL.clone(), vec![
                (enemy_paths::LEFT_STRAIGHT_DOWN.clone(), vec![0.5, 1., 1.5, 2., 2.5, 3.]),
                (enemy_paths::RIGHT_STRAIGHT_DOWN.clone(), vec![0.5, 1., 1.5, 2., 2.5, 3.]),
            ])]
        };
    }

    impl WaveSchemePrototype {
        pub fn compile(&self, seed: u64, difficulty: f32) -> CompiledWave {
            let mut enemies: Vec<(f32, Enemy)> = Vec::new();

            let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
            for (enemy_prototype, group_vec) in self.enemies.iter() {
                // each cycle represents an enemy group

                // they share the same cannon scheme defined as
                let mut cannons: Vec<Box<dyn CannonControllerInterface>> = Vec::new();

                // they use the same GraphicObjects
                let graphic_objects: GraphicObjects = 
                    enemy_prototype.graphic_objects_options.choose(&mut rng).unwrap().clone();

                for cannon_p_group in enemy_prototype.cannon_pits.iter() {
                    // each cycle represents a cannon group in an enemy group
                    let cannon_template = cannon::random_mapper(
                        rng.gen::<u64>(),
                        difficulty,
                    );
                    for each_cannon_p in cannon_p_group {
                        let mut each_p_cannon = cannon_template.clone();
                        each_p_cannon.set_p(*each_cannon_p);
                        cannons.push(each_p_cannon);
                    }
                }

                for (path, dts) in group_vec.iter() {
                    let enemy_construct = Enemy::new(
                        path.clone(),
                        cannons.clone(),
                        graphic_objects.clone(),
                        enemy_prototype.hitboxes.clone(),
                    );
                    for dt in dts.iter() {
                        enemies.push((*dt, enemy_construct.clone()));
                    }
                }
            }
            enemies.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            CompiledWave::new(VecDeque::from(enemies))
        }
    }

    pub fn random_mapper(seed: u64, difficulty: f32) -> CompiledWave {
        const scheme_size: u32 = 1;
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        match rng.gen_range(0, scheme_size) {
            0 => &LEFT_RIGHT_CHAINS,
            _ => unreachable!(),
        }.compile(rng.gen::<u64>(), difficulty)
    }
}

pub struct CompiledWave {
    // always sorted
    enemies: VecDeque<(f32, Enemy)>,
    timer: f32,
}

impl CompiledWave {
    pub fn new(enemies: VecDeque<(f32, Enemy)>) -> CompiledWave {
        CompiledWave {
            enemies: enemies,
            timer: 0.,
        }
    }

    pub fn tick(&mut self, dt: f32) -> VecDeque<Enemy> {
        let mut result = VecDeque::new();
        self.timer += dt;
        while match self.enemies.front() {
            None => false,
            Some(front) => front.0 < self.timer,
        } {
            let (_, enemy) = self.enemies.pop_front().unwrap();
            result.push_back(enemy);
        }
        result
    }
}

pub struct WaveGenerator {
    wave_cd: f32,
    wave_interval: f32,

    current_wave: Option<CompiledWave>,
    timer: f32,
}

impl WaveGenerator {
    pub fn new() -> WaveGenerator {
        WaveGenerator {
            wave_cd: 1.,
            wave_interval: 4.,

            current_wave: None,
            timer: 0.,
        }
    }

    // dummy
    pub fn tick(&mut self, mut dt: f32) -> VecDeque<Enemy> {
        let mut enemy_queue: VecDeque<Enemy> = VecDeque::new();
        // while is necessary, considering enemy generated at last frame and first frame
        // may appear in one tick call
        while dt > 0. {
            if self.wave_cd > dt {
                self.wave_cd -= dt;
                enemy_queue.extend(match &mut self.current_wave {
                    None => VecDeque::new(),
                    Some(compiled_wave) => compiled_wave.tick(dt),
                });
                dt = 0.;
            } else {
                enemy_queue.extend(match &mut self.current_wave {
                    None => VecDeque::new(),
                    Some(compiled_wave) => compiled_wave.tick(self.wave_cd),
                });
                dt -= self.wave_cd;
                self.wave_cd = self.wave_interval;
                self.current_wave = Some(wave_scheme_prototype::random_mapper(12345, 0.5));
            }
        }
        enemy_queue
    }

}
