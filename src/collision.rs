use crate::algebra::{linesegs_distance, Point2f};
use crate::bullet_pool::BulletPool;
use crate::destroy_effect::DestroyedObjects;
use crate::enemy_pool::EnemyPool;

pub trait CollisionPipeInterface {
    type Object;

    fn push(&mut self, object: Self::Object);
    fn pop(&mut self) -> Option<Self::Object>;
    fn len(&self) -> usize;
}

pub fn collision_enemy(
    enemy_pool: &mut EnemyPool,
    player_bullet_pool: &mut BulletPool,
    destroyed_objects: &mut DestroyedObjects,
) {
    // Time complexity notes:
    // O(l_e * l_pb)
    // player_bullet_pool < 10^2
    // enemy_pool < 10^2
    let enemy_len = enemy_pool.len();
    for _ in 0..enemy_len {
        let mut enemy = enemy_pool.pop().unwrap();
        let mut keep_enemy: bool = true;
        if let Some(enemy_last_p) = enemy.get_last_p() {
            if let Some(enemy_p) = enemy.get_p() {
                let bullet_len = player_bullet_pool.len();
                'bullet_loop: for _ in 0..bullet_len {
                    let bullet = player_bullet_pool.pop().unwrap();
                    let mut keep_bullet = true;

                    let bullet_p = bullet.get_p();
                    let bullet_last_p = bullet.get_last_p();
                    // we introduct collision flag here
                    // to prevent multiple collisions in many hitboxes in one bullet
                    let mut collision_flag: bool = false;
                    'hitbox_loop: for hitbox in enemy.get_hitboxes().iter() {
                        let dist = linesegs_distance(
                            enemy_p + hitbox.center,
                            enemy_last_p + hitbox.center,
                            bullet_p,
                            bullet_last_p,
                        );
                        //println!("{} {:?} {:?}", dist, bullet_p, bullet_last_p);
                        if dist < hitbox.r + bullet.get_r() {
                            keep_bullet = false;
                            collision_flag = true;
                            break 'hitbox_loop;
                        }
                    }
                    // if not collision, enemy will not take damage
                    if collision_flag && !enemy.damage(1.) {
                        keep_enemy = false;
                        break 'bullet_loop;
                    }

                    if keep_bullet {
                        player_bullet_pool.push(bullet);
                    }
                }
            }
        }
        if keep_enemy {
            enemy_pool.push(enemy);
        } else {
            destroyed_objects.push(enemy.get_shifted_graphic_objects());
        }
    }
}

pub fn collision_player(
    player_p: Point2f,
    player_last_p: Point2f,
    enemy_bullet_pool: &mut BulletPool,
) -> bool {
    const PLAYER_HITBOX_R: f32 = 5.;
    let bullet_len = enemy_bullet_pool.len();
    for _ in 0..bullet_len {
        let bullet = enemy_bullet_pool.pop().unwrap();
        let bullet_p = bullet.get_p();
        let bullet_last_p = bullet.get_last_p();
        let dist = linesegs_distance(player_p, player_last_p, bullet_p, bullet_last_p);
        //println!("{} {:?} {:?}", dist, bullet_p, bullet_last_p);
        if dist < PLAYER_HITBOX_R + bullet.get_r() {
            return true;
        }
        enemy_bullet_pool.push(bullet);
    }
    false
}
