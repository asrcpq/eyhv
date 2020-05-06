use crate::algebra::Point2f;
use crate::window_rect::{WINDOW_RECT, WINDOW_SIZE};

use rand::Rng;
use rand::SeedableRng;

pub mod enemy_paths {
    use super::EnemyPath;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref LEFT_STRAIGHT_DOWN: EnemyPath = EnemyPath::from_str("0.3 0 9 0.3 1 0");
        pub static ref MID_STRAIGHT_DOWN: EnemyPath = EnemyPath::from_str("0.5 0 9 0.5 1 0");
        pub static ref RIGHT_STRAIGHT_DOWN: EnemyPath = EnemyPath::from_str("0.7 0 9 0.7 1 0");
        pub static ref CLOCKWISE_ROLL: EnemyPath = EnemyPath::from_str("0.3 0 2 0.2 0.4 1 0.3 0.6 5 1 0.7 0");
        pub static ref COUNTERCLOCKWISE_ROLL: EnemyPath = EnemyPath::from_str("0.7 0 2 0.8 0.4 1 0.7 0.6 5 0 0.7 0");
        pub static ref LEFT_DOWN_OUT: EnemyPath = EnemyPath::from_str("0.2 0 4 0.3 0.5 3 0 0.7 0");
        pub static ref RIGHT_DOWN_OUT: EnemyPath = EnemyPath::from_str("0.8 0 4 0.7 0.5 3 1 0.7 0");
        pub static ref LEFT_RIGHT: EnemyPath = EnemyPath::from_str("0 0.1 6 1 0.2 0");
        pub static ref RIGHT_LEFT: EnemyPath = EnemyPath::from_str("1 0.1 6 0 0.2 0");
    }
}

#[derive(Clone)]
pub struct EnemyPath {
    // vertices and time takes for each edge
    // (route[-1] time is ignored)
    route: Vec<(Point2f, f32)>,

    // dynamic
    index: usize,
    timer: f32,
}

impl EnemyPath {
    pub fn generate_wanderer1(seed: u64) -> EnemyPath {
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
        let mut route: Vec<(Point2f, f32)> = Vec::new();
        const MAX_SHIFT: f32 = 0.2;
        let stay_height: f32 = rng.gen_range(0.2, 0.3);
        let start_p = Point2f::from_floats(rng.gen_range(0., 1.), 0.);
        let stay_p = Point2f::from_floats(
            rng.gen_range(0f32.max(start_p.x - MAX_SHIFT), 1f32.min(start_p.x + MAX_SHIFT)),
            stay_height,
        );
        let end_p = Point2f::from_floats(
            rng.gen_range(0f32.max(stay_p.x - MAX_SHIFT), 1f32.min(stay_p.x + MAX_SHIFT)),
            0.,
        );
        route.push((start_p * *WINDOW_SIZE, 1.));
        route.push((stay_p * *WINDOW_SIZE, 2.));
        route.push((stay_p * *WINDOW_SIZE, 1.));
        route.push((end_p * *WINDOW_SIZE, 0.));
        EnemyPath {
            route,
            index: 0,
            timer: 0.,
        }
    }

    pub fn from_str(line: &str) -> EnemyPath {
        let mut route = Vec::new();
        let splited = line
            .split_whitespace()
            .map(|x| x.parse::<f32>().expect("float parse fail"))
            .collect::<Vec<f32>>();
        let mut iter = splited.iter();
        while match iter.next() {
            Some(f1) => {
                // why copy trait does not work inside lazy_static??
                route.push((
                    Point2f::from_floats(*f1, *iter.next().unwrap()) * *WINDOW_SIZE
                        + WINDOW_RECT.lu,
                    *iter.next().unwrap(),
                ));
                true
            }
            None => false,
        } {}
        EnemyPath {
            route,
            index: 0,
            timer: 0.,
        }
    }

    pub fn tick(&mut self, dt_scaled: f32) -> Option<Point2f> {
        self.timer += dt_scaled;
        loop {
            let next_weight = self.route[self.index].1 - self.timer;
            if next_weight > 0. {
                return Some(
                    (self.route[self.index].0 * next_weight
                        + self.route[self.index + 1].0 * self.timer)
                        / self.route[self.index].1,
                );
            } else {
                if self.index == self.route.len() - 2 {
                    return None;
                }
                self.timer = -next_weight;
                self.index += 1;
            }
        }
    }
}
