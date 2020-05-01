use crate::algebra::Point2f;
use crate::window_rect::{WINDOW_RECT, WINDOW_SIZE};

pub mod enemy_paths {
    use super::EnemyPath;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref LEFT_STRAIGHT_DOWN: EnemyPath = EnemyPath::from_str("0.3 0. 10. 0.3 1. 0.");
    }
}

#[derive(Clone)]
pub struct EnemyPath {
    // vertices and time takes for each edge
    // (route[-1] time is ignored)
    route: Vec<(Point2f, f32)>,
    index: usize,

    // dynamic
    timer: f32,
}

impl EnemyPath {
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
                    Point2f::from_floats(*f1, *iter.next().unwrap()) * WINDOW_SIZE.clone()
                        + WINDOW_RECT.lu,
                    *iter.next().unwrap(),
                ));
                true
            }
            None => false,
        } {}
        EnemyPath {
            route: route,
            index: 0,
            timer: 0.,
        }
    }

    pub fn tick(&mut self, dt: f32) -> Option<Point2f> {
        self.timer += dt;
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
