use rand::SeedableRng;
use rand::Rng;
use rand_pcg;
// randomly split 1.0 into $count parts with each part > min_threshold
pub fn spliter(
    min_threshold: f32,
    count: u32,
    seed: u64,
) -> Vec<f32> {
    // gen_range should check that for us
    // if 1. < min_threshold * count {
    //     panic!("Invalid call to spliter!")
    // }

    // first reserve the space for threshold
    // then select count - 1 split points and sort them
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
    let max = 1.0  - min_threshold * count as f32;
    let mut split_points = (0..count - 1)
        .map(|x| rng.gen_range(0., max))
        .collect::<Vec<f32>>();
    split_points.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut result = Vec::new();
    let mut last_t: f32 = 0.;
    for split_point in split_points.iter() {
        result.push(split_point - last_t + min_threshold);
        last_t = *split_point;
    }
    result.push(max - last_t + min_threshold);
    result
}

#[cfg(test)]
mod test {
    use super::spliter;

    #[test]
    fn test_spliter() {
        let eps: f32 = 1e-5;
        const MIN_THRESHOLD: f32 = 0.2;
        const COUNT: u32 = 3;
        const SEED: u64 = 12345;
        let result = spliter(
            MIN_THRESHOLD,
            COUNT,
            SEED,
        );
        println!("{:?}", result);
        assert!(result.iter().fold(-1., |sum, x| sum + *x).abs() < eps);
        assert!(result.iter().fold(f32::INFINITY, |min, x| min.min(*x)) >= MIN_THRESHOLD);
    }

    #[test]
    #[should_panic]
    fn test_spliter_overflow() {
        let eps: f32 = 1e-5;
        const MIN_THRESHOLD: f32 = 0.2;
        const COUNT: u32 = 6;
        const SEED: u64 = 12345;
        let result = spliter(
            MIN_THRESHOLD,
            COUNT,
            SEED,
        );
        println!("{:?}", result);
    }
}
