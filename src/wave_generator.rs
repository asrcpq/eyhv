use std::collections::VecDeque;

use crate::enemy::Enemy;

use rand::Rng;
use rand::SeedableRng;

mod wave_scheme_prototype {
    use std::collections::VecDeque;

    use super::CompiledWave;
    use crate::cannon;
    use crate::cannon::CannonControllerInterface;
    use crate::enemy::enemy_prototype;
    use crate::enemy::Enemy;
    use crate::enemy_path::{enemy_paths, EnemyPath};
    use crate::graphic_object::GraphicObjects;

    use lazy_static::lazy_static;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use rand::SeedableRng;

    type GroupMemberSpatiotemporalInfo = Vec<(EnemyPath, Vec<f32>)>;
    #[derive(Clone)]
    pub struct WaveSchemePrototype {
        pub enemies: Vec<(
            enemy_prototype::EnemyPrototype,
            GroupMemberSpatiotemporalInfo,
        )>,
    }

    lazy_static! {
        static ref LEFT_DOWN_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::LEFT_DOWN_OUT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )]
        };
        static ref RIGHT_DOWN_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::RIGHT_DOWN_OUT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )]
        };
        static ref LEFT_RIGHT_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::LEFT_RIGHT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.]
                ),]
            )]
        };
        static ref RIGHT_LEFT_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::RIGHT_LEFT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )]
        };
        static ref CLOCKWISE_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::CLOCKWISE_ROLL.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )]
        };
        static ref COUNTERCLOCKWISE_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::COUNTERCLOCKWISE_ROLL.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )]
        };
        static ref LEFT_RIGHT_MEDIUM: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::MEDIUM.clone(),
                vec![(
                    enemy_paths::LEFT_STRAIGHT_DOWN.clone(),
                    vec![0.5],
                ),(
                    enemy_paths::RIGHT_STRAIGHT_DOWN.clone(),
                    vec![2.5],
                )]
            )]
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
                let graphic_objects: GraphicObjects = enemy_prototype
                    .graphic_objects_options
                    .choose(&mut rng)
                    .unwrap()
                    .clone();

                for cannon_p_group in enemy_prototype.cannon_pits.iter() {
                    // each cycle represents a cannon group in an enemy group
                    let cannon_template = cannon::random_mapper(rng.gen::<u64>(), difficulty);
                    for each_cannon_p in cannon_p_group {
                        let mut each_p_cannon = cannon_template.clone();
                        each_p_cannon.set_p(*each_cannon_p);
                        cannons.push(each_p_cannon);
                    }
                }

                for (path, dts) in group_vec.iter() {
                    let enemy_construct = Enemy::new(
                        path.clone(),
                        enemy_prototype.speed,
                        enemy_prototype.life,
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
        const SCHEME_SIZE: u32 = 7;
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        match rng.gen_range(0, SCHEME_SIZE) {
            0 => LEFT_DOWN_CHAIN.compile(rng.gen::<u64>(), difficulty),
            1 => RIGHT_DOWN_CHAIN.compile(rng.gen::<u64>(), difficulty),
            2 => LEFT_RIGHT_CHAIN.compile(rng.gen::<u64>(), difficulty),
            3 => RIGHT_LEFT_CHAIN.compile(rng.gen::<u64>(), difficulty),
            4 => LEFT_RIGHT_MEDIUM.compile(rng.gen::<u64>(), difficulty),
            5 => CLOCKWISE_CHAIN.compile(rng.gen::<u64>(), difficulty),
            6 => COUNTERCLOCKWISE_CHAIN.compile(rng.gen::<u64>(), difficulty),
            _ => unreachable!(),
        }
    }
}

pub struct CompiledWave {
    // always sorted
    enemies: VecDeque<(f32, Enemy)>,
    timer: f32,
}

impl CompiledWave {
    pub fn new(
        enemies: VecDeque<(f32, Enemy)>,
    ) -> CompiledWave {
        CompiledWave {
            enemies,
            timer: 0.,
        }
    }

    pub fn tick(&mut self, dt: f32) -> Option<VecDeque<Enemy>> {
        if self.enemies.is_empty() {
            None
        } else {
            let mut result = VecDeque::new();
            self.timer += dt;
            while match self.enemies.front() {
                None => false,
                Some(front) => front.0 < self.timer,
            } {
                let (_, enemy) = self.enemies.pop_front().unwrap();
                result.push_back(enemy);
            }
            Some(result)
        }
    }
}

pub struct WaveGenerator {
    wave_cd: f32,
    wave_interval: f32,
    rng: rand_pcg::Pcg64Mcg,
    wave_queue: VecDeque<CompiledWave>,
}

impl WaveGenerator {
    pub fn new(seed: u64) -> WaveGenerator {
        WaveGenerator {
            wave_cd: 1.,
            wave_interval: 2.,
            rng: rand_pcg::Pcg64Mcg::seed_from_u64(seed),
            wave_queue: VecDeque::new(),
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
                for _ in 0..self.wave_queue.len() {
                    let mut wave = self.wave_queue.pop_front().unwrap();
                    match wave.tick(dt) {
                        None => {},
                        Some(new_enemy_queue) => {
                            enemy_queue.extend(new_enemy_queue);
                            self.wave_queue.push_back(wave);
                        }
                    }
                }
                dt = 0.;
            } else {
                for _ in 0..self.wave_queue.len() {
                    let mut wave = self.wave_queue.pop_front().unwrap();
                    match wave.tick(self.wave_cd) {
                        None => {},
                        Some(new_enemy_queue) => {
                            enemy_queue.extend(new_enemy_queue);
                            self.wave_queue.push_back(wave);
                        }
                    }
                }
                dt -= self.wave_cd;
                self.wave_cd = self.wave_interval;
                self.wave_queue.push_back(wave_scheme_prototype::random_mapper(
                    self.rng.gen::<u64>(),
                    0.1,
                ));
            }
        }
        enemy_queue
    }
}
