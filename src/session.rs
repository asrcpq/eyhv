#[allow(unused_imports)]
use crate::collision::CollisionPipeInterface; // for memleak

use crate::background::Background;
use crate::bullet_pool::BulletPool;
use crate::canvas::Canvas;
use crate::collision::{collision_enemy, collision_player};
use crate::destroy_effect::DestroyedObjects;
use crate::difficulty_manager::DifficultyManager;
use crate::enemy_pool::EnemyPool;
use crate::fps_indicator::FpsIndicator;
use crate::graphic_object::{generate_thick_arc, GraphicObject, GraphicObjectsIntoIter};
use crate::key_state::KeyState;
use crate::player::Player;
use crate::record::Record;
use crate::slowdown_manager::SlowdownManager;
use crate::status_bar::StatusBar;
use crate::time_manager::TimeManager;
use crate::wave_generator::WaveGenerator;
use crate::window_rect::{SCALER, WINDOW_SIZE};

pub struct SessionGraphicObjectsIter {
	background_iter: GraphicObjectsIntoIter,
	player_iter: GraphicObjectsIntoIter,
	player_bullet_iter: GraphicObjectsIntoIter,
	destroyed_objects_iter: GraphicObjectsIntoIter,
	enemy_iter: GraphicObjectsIntoIter,
	enemy_bullet_iter: GraphicObjectsIntoIter,
	statusbar_iter: GraphicObjectsIntoIter,
	fpsindicator_iter: GraphicObjectsIntoIter,
}

impl Iterator for SessionGraphicObjectsIter {
	type Item = Box<dyn GraphicObject>;

	fn next(&mut self) -> Option<Box<dyn GraphicObject>> {
		match self.background_iter.next() {
			None => {}
			option => return option,
		}
		match self.player_bullet_iter.next() {
			None => {}
			option => return option,
		}
		match self.destroyed_objects_iter.next() {
			None => {}
			option => return option,
		}
		match self.enemy_iter.next() {
			None => {}
			option => return option,
		}
		match self.statusbar_iter.next() {
			None => {}
			option => return option,
		}
		match self.fpsindicator_iter.next() {
			None => {}
			option => return option,
		}
		match self.enemy_bullet_iter.next() {
			None => {}
			option => return option,
		}
		match self.player_iter.next() {
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
	destroyed_objects: DestroyedObjects,
	enemy_bullet_pool: BulletPool,

	record: Record,
	// ticks, operations
	replay: Option<(usize, usize)>,
	fast_replay: bool,

	difficulty_manager: DifficultyManager,
	wave_generator: WaveGenerator,

	// control
	key_state: KeyState,
	pause: bool,

	slowdown_manager: SlowdownManager,
	time_manager: TimeManager,
	status_bar: StatusBar,
	fps_indicator: FpsIndicator,
	background: Background,

	pub canvas: Canvas,

	#[allow(dead_code)]
	session_info: (u64, f32, f32, f32),
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
				Arg::with_name("difficulty drop")
					.short("D")
					.long("difficulty-drop")
					.takes_value(true)
					.help("difficulty drop on each hit"),
			)
			.arg(
				Arg::with_name("replay file")
					.short("f")
					.long("replay-file")
					.takes_value(true)
					.help("enable replay mode and load replay file(other args will be suppressed)"),
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
				println!("Using default start difficulty 0.24");
				0.2
			}
			Some(start_difficulty) => start_difficulty.parse::<f32>().unwrap(),
		};
		let difficulty_growth = match matches.value_of("difficulty growth") {
			None => {
				println!("Using default difficulty growth rate 0.0006");
				0.0006
			}
			Some(difficulty_growth) => difficulty_growth.parse::<f32>().unwrap(),
		};
		let difficulty_drop = match matches.value_of("difficulty drop") {
			None => 0.001,
			Some(difficulty_drop) => difficulty_drop.parse::<f32>().unwrap(),
		};
		let replay: Option<String> = matches.value_of("replay file").map(|s| s.to_string());
		let params;
		let mut record: Record;
		let replay = match replay {
			None => {
				record = Default::default();
				params = (seed, start_difficulty, difficulty_growth, difficulty_drop);
				record.params = params;
				None
			}
			Some(replay_file) => {
				record = Record::load(replay_file);
				params = record.params;
				Some((0, 0))
			}
		};
		Session {
			player: Player::new(),
			player_bullet_pool: BulletPool::new(),
			enemy_pool: EnemyPool::new(),
			destroyed_objects: DestroyedObjects::new(seed), //simply use the same seed
			enemy_bullet_pool: BulletPool::new(),
			record,
			replay,
			fast_replay: false,
			difficulty_manager: DifficultyManager::new(params.1, params.2, params.3),
			wave_generator: WaveGenerator::new(params.0),
			key_state: KeyState::new(),
			pause: false,
			slowdown_manager: SlowdownManager::new(),
			time_manager: TimeManager::new(),
			status_bar: StatusBar::new(params.1),
			fps_indicator: FpsIndicator::new(),
			background: Background::new(),
			canvas: Canvas::new((WINDOW_SIZE.x as i32, WINDOW_SIZE.y as i32), *SCALER),
			session_info: (seed, start_difficulty, difficulty_growth, difficulty_drop),
		}
	}

	fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
		SessionGraphicObjectsIter {
			player_iter: self.player.graphic_objects_iter(),
			player_bullet_iter: self.player_bullet_pool.graphic_objects_iter(),
			destroyed_objects_iter: self.destroyed_objects.graphic_objects_iter(),
			enemy_iter: self.enemy_pool.graphic_objects_iter(),
			enemy_bullet_iter: self.enemy_bullet_pool.graphic_objects_iter(),
			statusbar_iter: self.status_bar.graphic_objects_iter(),
			fpsindicator_iter: self.fps_indicator.graphic_objects_iter(),
			background_iter: self.background.graphic_objects_iter(),
		}
	}

	pub fn tick(&mut self, mut dt: f32) -> bool {
		if self.pause {
			std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 120));
			return true;
		}
		match self.replay {
			None => {
				// save unscaled time!
				self.record.dt_seq.push(dt);
			}
			Some((tn, opn)) => {
				if tn >= self.record.dt_seq.len() {
					self.exit();
					return false;
				}
				// this is not scaled!
				dt = self.record.dt_seq[tn];
				let mut new_opn = opn;
				loop {
					if new_opn >= self.record.operation.len() {
						self.exit();
						return false;
					}
					let (tick_n, key_id, updown) = self.record.operation[new_opn];
					if tick_n < tn {
						panic!("Unexpected operation skip!");
					}
					if tick_n > tn {
						break;
					}
					self.proc_key(key_id, updown);
					new_opn += 1;
				}
				self.replay = Some((tn + 1, new_opn));
			}
		}

		self.time_manager.set_state(self.slowdown_manager.tick(dt));
		let dt_scaled = dt * self.time_manager.update_and_get_dt_scaler(dt);

		let current_difficulty = self.difficulty_manager.get_difficulty();
		// not necessary to limit diffculty under 1.0
		if self.difficulty_manager.tick(dt_scaled) {
			self.background.send_message(format!(
				"     {: >3}    ",
				((1f32.min(current_difficulty) * 100.) as u32).to_string()
			));
			self.background.send_message("   LEVELUP  ".to_string());
		}

		self.player_bullet_pool.tick(dt_scaled);
		self.player_bullet_pool.extend(self.player.tick(
			dt_scaled,
			self.key_state.directions,
			self.time_manager.get_state(),
		));
		self.enemy_pool
			.extend(self.wave_generator.tick(dt_scaled, current_difficulty));
		self.enemy_bullet_pool.tick(dt_scaled);
		self.enemy_bullet_pool
			.extend(self.enemy_pool.tick(dt_scaled, self.player.get_p()));
		let slowdown_info = self.slowdown_manager.get_info();
		self.status_bar.tick(
			dt_scaled,
			current_difficulty,
			slowdown_info.0,
			slowdown_info.1,
			slowdown_info.2,
			self.player.get_p(),
		);
		self.fps_indicator.tick(dt);
		self.background.tick(dt_scaled, slowdown_info.2);
		self.destroyed_objects.tick(dt_scaled);
		collision_enemy(
			&mut self.enemy_pool,
			&mut self.player_bullet_pool,
			&mut self.destroyed_objects,
		);
		// no need to calculate collision if hit_reset-ing
		if !self.player.hit_reset()
			&& collision_player(
				self.player.get_p(),
				self.player.get_last_p(),
				&mut self.enemy_bullet_pool,
			) {
			self.player.hit();
			self.difficulty_manager.drop();
			self.status_bar.hit();
		}

		// memleak monitor
		// println!(
		//     "{} {} {} {}",
		//     self.player_bullet_pool.len(),
		//     self.enemy_bullet_pool.len(),
		//     self.enemy_pool.len(),
		//     self.destroyed_objects.len(),
		// );
		if !self.fast_replay {
			std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 120));
		}

		true
	}

	pub fn exit(&self) {
		println!(
			"Final difficulty: {}",
			self.difficulty_manager.get_difficulty()
		);
		self.record.save(".eyhv_replay".to_string());
	}

	fn toggle_pause(&mut self) {
		self.pause = !self.pause;
	}

	pub fn proc_key(&mut self, key_id: i8, updown: bool) {
		if key_id == 6 {
			self.toggle_pause();
			return;
		}
		if key_id == 7 {
			if self.replay != None {
				self.fast_replay = updown;
			}
			return;
		}
		if key_id == 8 {
			self.fps_indicator.switch();
			return;
		}
		if self.replay == None {
			self.record
				.operation
				.push((self.record.dt_seq.len(), key_id, updown));
		}
		if key_id == 4 {
			self.slowdown_manager.switch(updown);
		} else if key_id == 5 {
			self.player.switch_cannons(updown);
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
			1., 1., 1., 1., 75., 50., 100., 50., 100., 100., 85., 75., 65., 75., 50., 100.,
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
