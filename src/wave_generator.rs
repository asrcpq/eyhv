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
        next_wave: f32,
        difficulty_scaler: f32,
    }

    lazy_static! {
        static ref LEFT_DOWN_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::LEFT_DOWN_OUT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )],
            next_wave: 2.,
            difficulty_scaler: 1.,
        };
        static ref RIGHT_DOWN_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::RIGHT_DOWN_OUT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )],
            next_wave: 2.,
            difficulty_scaler: 1.,
        };
        static ref LEFT_RIGHT_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::LEFT_RIGHT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.]
                ),]
            )],
            next_wave: 2.,
            difficulty_scaler: 1.,
        };
        static ref RIGHT_LEFT_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::RIGHT_LEFT.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )],
            next_wave: 2.,
            difficulty_scaler: 1.,
        };
        static ref CLOCKWISE_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::CLOCKWISE_ROLL.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )],
            next_wave: 2.,
            difficulty_scaler: 0.8,
        };
        static ref COUNTERCLOCKWISE_CHAIN: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::SMALL.clone(),
                vec![(
                    enemy_paths::COUNTERCLOCKWISE_ROLL.clone(),
                    vec![0.5, 1., 1.5, 2., 2.5, 3.],
                ),]
            )],
            next_wave: 2.,
            difficulty_scaler: 0.8,
        };
        static ref LEFT_RIGHT_MEDIUM: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::MEDIUM.clone(),
                vec![
                    (enemy_paths::LEFT_STRAIGHT_DOWN.clone(), vec![0.5],),
                    (enemy_paths::RIGHT_STRAIGHT_DOWN.clone(), vec![2.5],)
                ]
            )],
            next_wave: 3.,
            difficulty_scaler: 1.2,
        };
        static ref MID_LARGE1: WaveSchemePrototype = WaveSchemePrototype {
            enemies: vec![(
                enemy_prototype::LARGE1.clone(),
                vec![(enemy_paths::MID_STRAIGHT_DOWN.clone(), vec![1.],)]
            )],
            next_wave: 4.,
            difficulty_scaler: 1.3,
        };
    }

    impl WaveSchemePrototype {
        fn generate_wanderer1(seed: u64) -> WaveSchemePrototype {
            let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
            WaveSchemePrototype {
                enemies: vec![(
                    enemy_prototype::SMALL.clone(),
                    (1..7)
                        .map(|x| {
                            (
                                EnemyPath::generate_wanderer1(rng.gen::<u64>()),
                                vec![x as f32 / 2.],
                            )
                        })
                        .collect(),
                )],
                next_wave: 2.,
                difficulty_scaler: 1.,
            }
        }

        fn generate_wanderer2(seed: u64) -> WaveSchemePrototype {
            let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
            WaveSchemePrototype {
                enemies: vec![(
                    enemy_prototype::SMALL.clone(),
                    (1..7)
                        .map(|x| {
                            (
                                EnemyPath::generate_wanderer2(rng.gen::<u64>()),
                                vec![x as f32 / 2.],
                            )
                        })
                        .collect(),
                )],
                next_wave: 2.,
                difficulty_scaler: 1.,
            }
        }

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
                    let cannon_template = cannon::random_mapper(
                        rng.gen::<u64>(),
                        difficulty * self.difficulty_scaler,
                        enemy_prototype.cannon_pool,
                    );
                    for each_cannon_p in cannon_p_group {
                        let mut each_p_cannon = cannon_template.clone();
                        each_p_cannon.set_p(*each_cannon_p);
                        cannons.push(each_p_cannon);
                    }
                }

                for (path, dts) in group_vec.iter() {
                    for dt in dts.iter() {
                        let mut cloned_cannons = cannons.clone();
                        for cannon in cloned_cannons.iter_mut() {
                            cannon.set_rng(rng.gen::<u64>());
                        }
                        enemies.push((
                            *dt,
                            Enemy::new(
                                path.clone(),
                                enemy_prototype.speed,
                                enemy_prototype.life,
                                cloned_cannons,
                                graphic_objects.clone(),
                                enemy_prototype.hitboxes.clone(),
                            ),
                        ));
                    }
                }
            }
            enemies.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            CompiledWave::new(VecDeque::from(enemies), self.next_wave)
        }
    }

    pub fn random_mapper(
        seed: u64,
        difficulty: f32,
        last: Option<u32>,
    ) -> (Option<u32>, CompiledWave) {
        const SCHEME_SIZE: u32 = 10;
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        let mut type_id = rng.gen_range(0, SCHEME_SIZE - 1);
        // skip randomly generated path
        if type_id != 0 {
            if let Some(last) = last {
                if type_id >= last {
                    type_id += 1;
                    type_id %= SCHEME_SIZE;
                }
            }
        }
        (
            Some(type_id),
            match type_id {
                0 => WaveSchemePrototype::generate_wanderer1(rng.gen::<u64>())
                    .compile(rng.gen::<u64>(), difficulty),
                1 => WaveSchemePrototype::generate_wanderer2(rng.gen::<u64>())
                    .compile(rng.gen::<u64>(), difficulty),
                2 => LEFT_DOWN_CHAIN.compile(rng.gen::<u64>(), difficulty),
                3 => RIGHT_DOWN_CHAIN.compile(rng.gen::<u64>(), difficulty),
                4 => LEFT_RIGHT_CHAIN.compile(rng.gen::<u64>(), difficulty),
                5 => RIGHT_LEFT_CHAIN.compile(rng.gen::<u64>(), difficulty),
                6 => LEFT_RIGHT_MEDIUM.compile(rng.gen::<u64>(), difficulty),
                7 => CLOCKWISE_CHAIN.compile(rng.gen::<u64>(), difficulty),
                8 => COUNTERCLOCKWISE_CHAIN.compile(rng.gen::<u64>(), difficulty),
                9 => MID_LARGE1.compile(rng.gen::<u64>(), difficulty),
                _ => unreachable!(),
            },
        )
    }
}

pub struct CompiledWave {
    // always sorted
    enemies: VecDeque<(f32, Enemy)>,
    timer: f32,
    next_wave: f32,
}

impl CompiledWave {
    pub fn new(enemies: VecDeque<(f32, Enemy)>, next_wave: f32) -> CompiledWave {
        CompiledWave {
            enemies,
            timer: 0.,
            next_wave,
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
    rng: rand_pcg::Pcg64Mcg,
    wave_queue: VecDeque<CompiledWave>,

    last_type: Option<u32>,

    handle: Option<std::thread::JoinHandle<(Option<u32>, CompiledWave)>>,
    //handle: Option<std::thread::JoinHandle<u32>>,
}

impl WaveGenerator {
    pub fn new(seed: u64) -> WaveGenerator {
        WaveGenerator {
            wave_cd: 1.,
            rng: rand_pcg::Pcg64Mcg::seed_from_u64(seed),
            wave_queue: VecDeque::new(),
            last_type: None,
            handle: None,
        }
    }

    pub fn tick(&mut self, mut dt: f32, difficulty: f32) -> VecDeque<Enemy> {
        let mut enemy_queue: VecDeque<Enemy> = VecDeque::new();
        // while is necessary, considering enemy generated at last frame and first frame
        // may appear in one tick call

        // print difficulty to stdout
        // use std::io::Write;
        // print!("{}\r", self.difficulty);
        // std::io::stdout().flush().unwrap();

        while dt > 0. {
            if self.wave_cd > dt {
                self.wave_cd -= dt;
                for _ in 0..self.wave_queue.len() {
                    let mut wave = self.wave_queue.pop_front().unwrap();
                    match wave.tick(dt) {
                        None => {}
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
                        None => {}
                        Some(new_enemy_queue) => {
                            enemy_queue.extend(new_enemy_queue);
                            self.wave_queue.push_back(wave);
                        }
                    }
                }
                dt -= self.wave_cd;

                let seed = self.rng.gen::<u64>();
                let last_type = self.last_type;

                let (last_type, compiled_wave) = if self.handle.is_none() {
                    println!("Hard wait on wave generation");
                    wave_scheme_prototype::random_mapper(seed, difficulty, last_type)
                } else {
                    self.handle.take().unwrap().join().unwrap()
                };
                self.last_type = last_type;
                self.wave_queue.push_back(compiled_wave);
                self.wave_cd = self.wave_queue.back().unwrap().next_wave * (1. - difficulty / 1.6);

                self.handle = Some(std::thread::spawn(move || {
                    wave_scheme_prototype::random_mapper(seed, difficulty, last_type)
                }));
            }
        }
        enemy_queue
    }
}
