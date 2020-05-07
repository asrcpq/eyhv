use crate::background::Background;
use crate::bullet_pool::BulletPool;
use crate::canvas::Canvas;
use crate::collision::{collision_enemy, collision_player};
use crate::enemy_pool::EnemyPool;
use crate::graphic_object::{generate_thick_arc, GraphicObject, GraphicObjectsIntoIter};
use crate::key_state::KeyState;
use crate::player::Player;
use crate::slowdown_manager::SlowdownManager;
use crate::status_bar::StatusBar;
use crate::time_manager::TimeManager;
use crate::wave_generator::WaveGenerator;
use crate::window_rect::WINDOW_SIZE;

pub struct SessionGraphicObjectsIter {
    player_iter: GraphicObjectsIntoIter,
    player_bullet_iter: GraphicObjectsIntoIter,
    enemy_iter: GraphicObjectsIntoIter,
    enemy_bullet_iter: GraphicObjectsIntoIter,
    statusbar_iter: GraphicObjectsIntoIter,
    background_iter: GraphicObjectsIntoIter,
}

impl Iterator for SessionGraphicObjectsIter {
    type Item = Box<dyn GraphicObject>;

    fn next(&mut self) -> Option<Box<dyn GraphicObject>> {
        match self.player_iter.next() {
            None => {}
            option => return option,
        }
        match self.player_bullet_iter.next() {
            None => {}
            option => return option,
        }
        match self.enemy_iter.next() {
            None => {}
            option => return option,
        }
        match self.enemy_bullet_iter.next() {
            None => {}
            option => return option,
        }
        match self.statusbar_iter.next() {
            None => {}
            option => return option,
        }
        match self.background_iter.next() {
            None => {}
            option => return option,
        }
        None
    }
}

pub struct Session {
    player: Player,
    player_bullet_pool: BulletPool,
    enemy_pool: EnemyPool,
    enemy_bullet_pool: BulletPool,

    difficulty: f32,
    difficulty_growth: f32,

    wave_generator: WaveGenerator,

    // control
    key_state: KeyState,
    pause: bool,

    slowdown_manager: SlowdownManager,
    time_manager: TimeManager,
    status_bar: StatusBar,
    background: Background,

    pub canvas: Canvas,

    #[allow(dead_code)]
    session_info: (u64, f32, f32, f32, f32),
}

impl Session {
    pub fn new() -> Session {
        use clap::{App, Arg};
        let matches = App::new("eyhv: Shoot 'em up game inspired by PARSEC47")
            .arg(
                Arg::with_name("seed")
                    .short("s")
                    .long("seed")
                    .takes_value(true)
                    .help("random seed used"),
            )
            .arg(
                Arg::with_name("start difficulty")
                    .short("d")
                    .long("start-difficulty")
                    .takes_value(true)
                    .help("difficulty at start"),
            )
            .arg(
                Arg::with_name("difficulty growth")
                    .short("g")
                    .long("difficulty-growth")
                    .takes_value(true)
                    .help("difficulty growth per second"),
            )
            .arg(
                Arg::with_name("health max")
                    .short("h")
                    .long("health-max")
                    .takes_value(true)
                    .help("max healyh"),
            )
            .arg(
                Arg::with_name("health regen")
                    .short("r")
                    .long("health-rengen")
                    .takes_value(true)
                    .help("regeneration of health per second"),
            )
            .get_matches();
        let seed = match matches.value_of("seed") {
            None => {
                use rand::Rng;
                use rand::SeedableRng;
                let mut rng = rand_pcg::Pcg64Mcg::from_entropy();
                let seed = rng.gen::<u64>();
                println!("Seed generated: {}", seed);
                seed
            }
            Some(seed) => seed.parse::<u64>().unwrap(),
        };
        let start_difficulty = match matches.value_of("start difficulty") {
            None => {
                println!("Using default start difficulty 0.2");
                0.2
            }
            Some(start_difficulty) => start_difficulty.parse::<f32>().unwrap(),
        };
        let difficulty_growth = match matches.value_of("difficulty growth") {
            None => {
                println!("Using default difficulty growth rate 0.001");
                0.001
            }
            Some(difficulty_growth) => difficulty_growth.parse::<f32>().unwrap(),
        };
        let health_max = match matches.value_of("health max") {
            None => 10.,
            Some(health_max) => health_max.parse::<f32>().unwrap(),
        };
        let health_regen = match matches.value_of("health regen") {
            None => 0.07,
            Some(health_regen) => health_regen.parse::<f32>().unwrap(),
        };
        Session {
            player: Player::new(health_max, health_regen),
            player_bullet_pool: BulletPool::new(),
            enemy_pool: EnemyPool::new(),
            enemy_bullet_pool: BulletPool::new(),
            difficulty: start_difficulty,
            difficulty_growth,
            wave_generator: WaveGenerator::new(seed),
            key_state: KeyState::new(),
            pause: false,
            slowdown_manager: SlowdownManager::new(),
            time_manager: TimeManager::new(),
            status_bar: StatusBar::new(),
            background: Background::new(),
            canvas: Canvas::new((WINDOW_SIZE.x as u32, WINDOW_SIZE.y as u32)),
            session_info: (
                seed,
                start_difficulty,
                difficulty_growth,
                health_max,
                health_regen,
            ),
        }
    }

    fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_iter: self.player.graphic_objects_iter(),
            player_bullet_iter: self.player_bullet_pool.graphic_objects_iter(),
            enemy_iter: self.enemy_pool.graphic_objects_iter(),
            enemy_bullet_iter: self.enemy_bullet_pool.graphic_objects_iter(),
            statusbar_iter: self.status_bar.graphic_objects_iter(),
            background_iter: self.background.graphic_objects_iter(),
        }
    }

    pub fn tick(&mut self, mut dt: f32) {
        if self.pause {
            return;
        }
        self.time_manager.set_state(self.slowdown_manager.tick(dt));
        dt *= self.time_manager.update_and_get_dt_scaler(dt);

        let player_health = self.player.get_health_percent();
        // difficulty added before dt changed
        // not necessary to limit diffculty under 1.0
        self.difficulty += self.difficulty_growth * dt * (player_health > 0.99) as i32 as f32;

        self.player_bullet_pool.tick(dt);
        self.player_bullet_pool.extend(self.player.tick(
            dt,
            self.key_state.directions,
            self.time_manager.get_state(),
        ));
        self.enemy_pool
            .extend(self.wave_generator.tick(dt, self.difficulty));
        self.enemy_bullet_pool.tick(dt);
        self.enemy_bullet_pool
            .extend(self.enemy_pool.tick(dt, self.player.get_p()));
        let slowdown_info = self.slowdown_manager.get_info();
        self.status_bar.tick(
            dt,
            player_health,
            slowdown_info.0,
            slowdown_info.1,
            slowdown_info.2,
            self.player.get_p(),
        );
        self.background.tick(dt);
        collision_enemy(&mut self.enemy_pool, &mut self.player_bullet_pool);
        if !self.player.hit_reset()
            && collision_player(
                self.player.get_p(),
                self.player.get_last_p(),
                &mut self.enemy_bullet_pool,
            )
            && !self.player.hit()
        {
            println!("Died! final difficulty: {}", self.difficulty);
        }

        // memleak monitor
        // println!(
        //     "{} {} {}",
        //     self.player_bullet_pool.len(),
        //     self.enemy_bullet_pool.len(),
        //     self.enemy_pool.len()
        // );
    }

    pub fn exit(&self) {
        println!("Early exit! final difficulty: {}", self.difficulty);
    }

    fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        if key_id == 4 {
            self.slowdown_manager.switch(updown);
        } else if key_id == 5 {
            self.player.switch_cannons(updown);
        } else if key_id == 6 {
            self.toggle_pause();
        } else {
            self.key_state.proc_key(key_id, updown);
        }
    }

    pub fn render(&mut self) {
        if !self.pause {
            self.canvas.flush();
            for graphic_object in self.graphic_object_iter() {
                graphic_object.render(&mut self.canvas);
            }
        }
    }

    #[allow(dead_code)]
    pub fn test_render(&mut self) {
        use crate::algebra::Point2f;
        use crate::graphic_object::{LineSegs2f, Polygon2f};
        self.canvas.flush();
        let split = 90;
        for k in 0..split {
            let ang = std::f32::consts::PI * 2. / split as f32 * -k as f32;
            LineSegs2f::from_floats(vec![
                1.,
                1.,
                1.,
                1.,
                250.,
                250.,
                250. + 200. * ang.cos(),
                250. + 200. * ang.sin(),
            ])
            .render(&mut self.canvas);
        }
        Polygon2f::from_floats(vec![
            1., 1., 1., 1., 100., 50., 50., 100., 100., 150., 150., 100.,
        ])
        .render(&mut self.canvas);
        for graphic_object in generate_thick_arc(
            Point2f::from_floats(200., 200.),
            (83., 95.),
            (0., 6.),
            Some([1., 0.5, 0.5, 1.]),
            Some([1., 0.4, 0.4, 0.3]),
        )
        .into_iter()
        {
            graphic_object.render(&mut self.canvas);
        }
    }
}
