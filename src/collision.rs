use crate::algebra::{Point2f, linesegs_distance};
use crate::bullet_pool::BulletPool;
use crate::enemy_pool::EnemyPool;

pub trait CollisionPipeInterface {
    type Object;

    fn push(&mut self, object: Self::Object);
    fn pop(&mut self) -> Option<Self::Object>;
    fn len(&self) -> usize;
}

pub fn collision_enemy(enemy_pool: &mut EnemyPool, player_bullet_pool: &mut BulletPool) {
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

pub fn collision_player(player_p: Point2f, player_last_p: Point2f, enemy_bullet_pool: &mut BulletPool) {
    const player_hitbox_r: f32 = 5.;
    let bullet_len = enemy_bullet_pool.len();
    'bullet_loop: for _ in 0..bullet_len {
        let bullet = enemy_bullet_pool.pop().unwrap();
        if let Some(bullet_p) = bullet.get_p() {
            if let Some(bullet_last_p) = bullet.get_last_p() {
                let dist = linesegs_distance(
                    player_p,
                    player_last_p,
                    bullet_p,
                    bullet_last_p,
                );
                //println!("{} {:?} {:?}", dist, bullet_p, bullet_last_p);
                if dist < player_hitbox_r + bullet.get_r() {
                    println!("BOOM!");
                    break 'bullet_loop;
                }
            }
        }
        enemy_bullet_pool.push(bullet);
    }
}
