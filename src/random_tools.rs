use rand::Rng;
use rand::SeedableRng;

// feed n groups
pub fn simple_try<F>(
	mut try_times: u32,
	evaluate: F,
	range: Vec<(f32, f32)>,
	correlation: f32, // 0-1 correlation between variables
	mut expect_difficulty: f32,
	seed: u64,
) -> Vec<f32>
where
	F: Fn(&Vec<f32>) -> f32,
{
	let mut mut_range = range.clone();

	let mut k = 0.5; // correlation component initial value
	let mut k_down = 0.;
	let mut k_up = 1.;

	expect_difficulty = evaluate(
		&range
			.iter()
			.map(|(x, y)| x * (1. - expect_difficulty) + y * expect_difficulty)
			.collect(),
	);

	let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);
	loop {
		let generated: Vec<f32> = mut_range
			.iter()
			.map(|(x, y)| {
				if x > y {
					rng.gen_range(y, x)
				} else if x < y {
					rng.gen_range(x, y)
				} else {
					*x // x == y happens occasionally
				}
			})
			.collect();
		try_times -= 1;
		if try_times == 0 {
			break generated
				.iter()
				.zip(range.iter().map(|(x, y)| x * (1. - k) + y * k))
				.map(|(x, y)| x * (1. - correlation) + y * correlation)
				.collect();
		}
		let generated_error = evaluate(&generated) - expect_difficulty;
		if generated_error > 0. {
			mut_range = mut_range
				.iter()
				.zip(generated.iter())
				.map(|((x, _), z)| (*x, *z))
				.collect();
		} else {
			mut_range = mut_range
				.iter()
				.zip(generated.iter())
				.map(|((_, y), z)| (*z, *y))
				.collect();
		}

		// correlation component
		if evaluate(&range.iter().map(|(x, y)| x * (1. - k) + y * k).collect()) - expect_difficulty
			> 0.
		{
			k_up = k;
		} else {
			k_down = k;
		}
		k = (k_up + k_down) / 2.;
	}
}

#[cfg(test)]
mod test {
	use super::simple_try;

	#[test]
	fn test_simple_try() {
		let result = simple_try(
			10,
			|x| x[0] + x[1],
			vec![(0., 1.), (0., 1.)],
			0.5,
			1.5,
			12345,
		);
		println!("{:?}", result);
		assert!(result[0] >= 0. && result[0] <= 1.);
		assert!(result[1] >= 0. && result[1] <= 1.);
	}
}
