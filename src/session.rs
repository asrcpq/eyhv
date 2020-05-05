use crate::bullet_pool::BulletPool;
use crate::collision::{collision_enemy, collision_player};
use crate::enemy_pool::EnemyPool;
use crate::graphic_object::{GraphicObject, GraphicObjectsIntoIter};
use crate::key_state::KeyState;
use crate::player::Player;
use crate::time_manager::TimeManager;
use crate::wave_generator::WaveGenerator;
use crate::window_rect::WINDOW_SIZE;
use crate::canvas::Canvas;

pub struct SessionGraphicObjectsIter {
    player_iter: GraphicObjectsIntoIter,
    player_bullet_iter: GraphicObjectsIntoIter,
    enemy_iter: GraphicObjectsIntoIter,
    enemy_bullet_iter: GraphicObjectsIntoIter,
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
        None
    }
}

pub struct Session {
    player: Player,
    player_bullet_pool: BulletPool,
    enemy_pool: EnemyPool,
    enemy_bullet_pool: BulletPool,

    wave_generator: WaveGenerator,

    // control
    key_state: KeyState,
    pause: bool,

    time_manager: TimeManager,

    pub canvas: Canvas,
}

impl Session {
    pub fn new() -> Session {
        Session {
            player: Player::new(),
            player_bullet_pool: BulletPool::new(),
            enemy_pool: EnemyPool::new(),
            enemy_bullet_pool: BulletPool::new(),
            wave_generator: WaveGenerator::new(12345),
            key_state: KeyState::new(),
            pause: false,
            time_manager: TimeManager::new(),
            canvas: Canvas::new((WINDOW_SIZE.x as u32, WINDOW_SIZE.y as u32)),
        }
    }

    fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_iter: self.player.graphic_objects_iter(),
            player_bullet_iter: self.player_bullet_pool.graphic_objects_iter(),
            enemy_iter: self.enemy_pool.graphic_objects_iter(),
            enemy_bullet_iter: self.enemy_bullet_pool.graphic_objects_iter(),
        }
    }

    pub fn tick(&mut self, mut dt: f32) {
        if self.pause {
            return;
        }
        dt *= self.time_manager.update_and_get_dt_scaler(dt);
        self.player_bullet_pool.tick(dt);
        self.player_bullet_pool
            .extend(self.player.tick(dt, self.key_state.directions));
        self.enemy_pool.extend(self.wave_generator.tick(dt));
        self.enemy_bullet_pool.tick(dt);
        self.enemy_bullet_pool
            .extend(self.enemy_pool.tick(dt, self.player.get_p()));
        collision_enemy(&mut self.enemy_pool, &mut self.player_bullet_pool);
        if !self.player.hit_reset() && collision_player(
            self.player.get_p(),
            self.player.get_last_p(),
            &mut self.enemy_bullet_pool,
        ) {
            self.player.hit();
        }

        // memleak monitor
        // println!(
        //     "{} {} {}",
        //     self.player_bullet_pool.len(),
        //     self.enemy_bullet_pool.len(),
        //     self.enemy_pool.len()
        // );
    }

    fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        if key_id == 4 {
            self.time_manager.set_state(updown);
        } else if key_id == 5 {
            self.player.switch_cannons(updown);
        } else if key_id == 6 {
            self.toggle_pause();
        } else {
            self.key_state.proc_key(key_id, updown);
        }
    }

    pub fn render(&mut self) {
        self.canvas.flush();
        for graphic_object in self.graphic_object_iter() {
            graphic_object.render(&mut self.canvas);
        }
    }

    pub fn test_render(&mut self) {
        use crate::graphic_object::LineSegs2f;
        self.canvas.flush();
        let split = 90;
        for k in 0..split {
            let ang = 3.1415926 * 2. / split as f32 * -k as f32;
            LineSegs2f::from_floats(vec![1., 1., 1., 1., 250., 250.,
                250. + 200. * ang.cos(),
                250. + 200. * ang.sin(),
            ]).render(&mut self.canvas);
        }
    }
}
