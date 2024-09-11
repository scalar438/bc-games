pub fn calc_bc(guess: &str, hidden: &str) -> (i32, i32) {
	let mut guess_digits = [false; 10];
	let mut hidden_digits = [false; 10];
	let mut bulls = 0;
	for (gc, hc) in guess.chars().zip(hidden.chars()) {
		if gc == hc {
			bulls += 1;
		} else {
			guess_digits[(gc as usize) - ('0' as usize)] = true;
			hidden_digits[(hc as usize) - ('0' as usize)] = true;
		}
	}
	let mut cows = 0;
	for (f1, f2) in guess_digits.iter().zip(hidden_digits.iter()) {
		if *f1 && *f2 {
			cows += 1;
		}
	}
	(bulls, cows)
}

pub fn gen_values(n: i32) -> Vec<String> {
	let m = {
		let mut x = 1;
		for _ in 0..n {
			x *= 10;
		}
		x
	};
	(0..m)
		.filter_map(|mut x| {
			let mut s = String::new();
			let mut hs = [false; 10];
			for _ in 0..n {
				if hs[x % 10] {
					return None;
				}
				hs[x % 10] = true;
				s.push((((x % 10) as u8) + ('0' as u8)) as char);
				x /= 10;
			}
			Some(s)
		})
		.collect()
}

#[test]
fn test_calc_bc() {
	assert_eq!(calc_bc("0123", "0432"), (1, 2));
	assert_eq!(calc_bc("1234", "5678"), (0, 0));
	assert_eq!(calc_bc("12304", "43210"), (0, 5));
	assert_eq!(calc_bc("123456", "123456"), (6, 0));
	assert_eq!(calc_bc("1234", "7893"), (0, 1));
}
