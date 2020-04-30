use crate::algebra::{linesegs_distance, Rect2f};
use crate::bullet_pool::BulletPool;
use crate::collision_pipe_interface::CollisionPipeInterface;
use crate::enemy_pool::EnemyPool;
use crate::graphic_object::{GraphicObject, GraphicObjectsIntoIter};
use crate::key_state::KeyState;
use crate::player::Player;
use crate::time_manager::TimeManager;
use crate::wave_generator::WaveGenerator;

pub struct SessionGraphicObjectsIter {
    player_iter: GraphicObjectsIntoIter,
    player_bullet_iter: GraphicObjectsIntoIter,
    enemy_iter: GraphicObjectsIntoIter,
    enemy_bullet_iter: GraphicObjectsIntoIter,
}

impl Iterator for SessionGraphicObjectsIter {
    type Item = GraphicObject;

    fn next(&mut self) -> Option<GraphicObject> {
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

fn collision_enemy(enemy_pool: &mut EnemyPool, player_bullet_pool: &mut BulletPool) {
    // Time complexity notes:
    // O(l_e * l_pb)
    // player_bullet_pool < 10^2
    // enemy_pool < 10^2
    let enemy_len = enemy_pool.len();
    for _ in 0..enemy_len {
        let enemy = enemy_pool.pop().unwrap();
        let mut keep_enemy: bool = true;
        if let Some(enemy_last_p) = enemy.get_last_p() {
            if let Some(enemy_p) = enemy.get_p() {
                let bullet_len = player_bullet_pool.len();
                'bullet_loop: for _ in 0..bullet_len {
                    let bullet = player_bullet_pool.pop().unwrap();
                    if let Some(bullet_p) = bullet.get_p() {
                        if let Some(bullet_last_p) = bullet.get_last_p() {
                            for hitbox in enemy.get_hitboxes().iter() {
                                let dist = linesegs_distance(
                                    enemy_p + hitbox.center,
                                    enemy_last_p + hitbox.center,
                                    bullet_p,
                                    bullet_last_p,
                                );
                                //println!("{} {:?} {:?}", dist, bullet_p, bullet_last_p);
                                if dist < hitbox.r + bullet.get_r() {
                                    keep_enemy = false;
                                    println!("BANG!");
                                    break 'bullet_loop;
                                }
                            }
                        }
                    }
                    player_bullet_pool.push(bullet);
                }
            }
        }
        if keep_enemy {
            enemy_pool.push(enemy);
        }
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

    time_manager: TimeManager,
}

impl Session {
    pub fn new() -> Session {
        Session {
            player: Player::new(),
            player_bullet_pool: BulletPool::new(),
            enemy_pool: EnemyPool::new(),
            enemy_bullet_pool: BulletPool::new(),
            wave_generator: WaveGenerator::new(),
            key_state: KeyState::new(),
            time_manager: TimeManager::new(),
        }
    }

    pub fn graphic_object_iter(&self) -> SessionGraphicObjectsIter {
        SessionGraphicObjectsIter {
            player_iter: self.player.graphic_objects_iter(),
            player_bullet_iter: self.player_bullet_pool.graphic_objects_iter(),
            enemy_iter: self.enemy_pool.graphic_objects_iter(),
            enemy_bullet_iter: self.enemy_bullet_pool.graphic_objects_iter(),
        }
    }

    pub fn tick(&mut self, mut dt: f32) {
        dt *= self.time_manager.update_and_get_dt_scaler(dt);
        self.player_bullet_pool.tick(dt);
        self.player_bullet_pool
            .extend(self.player.tick(dt, &self.key_state.directions));
        self.enemy_pool.extend(self.wave_generator.tick(dt));
        self.enemy_bullet_pool.tick(dt);
        self.enemy_bullet_pool
            .extend(self.enemy_pool.tick(dt, self.player.get_p()));
        collision_enemy(&mut self.enemy_pool, &mut self.player_bullet_pool);

        // memleak monitor
        // println!(
        //     "{} {} {}",
        //     self.player_bullet_pool.len(),
        //     self.enemy_bullet_pool.len(),
        //     self.enemy_pool.len()
        // );
    }

    pub fn proc_key(&mut self, key_id: i8, updown: bool) {
        if key_id == 4 {
            self.time_manager.set_state(updown);
        } else if key_id == 5 {
            self.player.switch_cannons(updown);
        } else {
            self.key_state.proc_key(key_id, updown);
        }
    }
}
