use rand::Rng;
use rand::SeedableRng;

// feed n groups
pub fn simple_try<F>(
    mut try_times: u32,
    evaluate: F,
    mut range: Vec<(f32, f32)>,
    mut expect_difficulty: f32,
    seed: u64,
) -> Vec<f32>
where
    F: Fn(&Vec<f32>) -> f32,
{
    let min = evaluate(&range.iter().map(|(x, _)| *x).collect());
    let max = evaluate(&range.iter().map(|(_, y)| *y).collect());
    expect_difficulty = expect_difficulty * (max - min) + min;

    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
    loop {
        let generated = range
            .iter()
            .map(|(x, y)| if x > y { (y, x) } else { (x, y) })
            .map(|(begin, end)| rng.gen_range(begin, end))
            .collect();
        try_times -= 1;
        if try_times == 0 {
            break generated;
        }
        let generated_error = evaluate(&generated) - expect_difficulty;
        if generated_error > 0. {
            range = range
                .iter()
                .zip(generated.iter())
                .map(|((x, _), z)| (*x, *z))
                .collect();
        } else {
            range = range
                .iter()
                .zip(generated.iter())
                .map(|((_, y), z)| (*z, *y))
                .collect();
        }
    }
}

#[cfg(test)]
mod test {
    use super::simple_try;

    #[test]
    fn test_simple_try() {
        let result = simple_try(10, |x| x[0] + x[1], vec![(0., 1.), (0., 1.)], 1.5, 12345);
        println!("{:?}", result);
        assert!(result[0] >= 0. && result[0] <= 1.);
        assert!(result[1] >= 0. && result[1] <= 1.);
    }
}
