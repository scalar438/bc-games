use std::path::Iter;

pub struct GameParams {
	num_len: u8,
	has_repetitions: bool,
	digits_number: u8,
}

struct Numbers {}

impl Iterator for Numbers {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
	if a.len() < 2 {
		return false;
	}
	let mut i = a.len() - 2;
	while a[i] >= a[i + 1] {
		if i == 0 {
			return false;
		}
		i -= 1;
	}
	let mut j = a.len() - 1;
	while a[i] >= a[j] {
		j -= 1;
	}
	a.swap(i, j);
	a[i + 1..].reverse();
	return true;
}

fn increase_vector(a: &mut [i32], maxval: i32) -> bool {
	let mut l = a.len();
	loop {
		if l == 0 {
			return false;
		}
		l -= 1;
		if a[l] == maxval {
			a[l] = 0;
		} else {
			a[l] += 1;
			break;
		}
	}
	return true;
}

#[test]
fn test_permutation() {
	{
		let mut a = [4];
		assert!(!next_permutation(&mut a));
	}
	{
		let mut a = [1, 2];
		assert!(next_permutation(&mut a));
		assert_eq!(a, [2, 1]);
		assert!(!next_permutation(&mut a));
	}
	{
		let mut a = [1, 2, 3];
		assert!(next_permutation(&mut a));
		assert_eq!(a, [1, 3, 2]);
		assert!(next_permutation(&mut a));
		assert_eq!(a, [2, 1, 3]);
		assert!(next_permutation(&mut a));
		assert_eq!(a, [2, 3, 1]);
		assert!(next_permutation(&mut a));
		assert_eq!(a, [3, 1, 2]);
		assert!(next_permutation(&mut a));
		assert_eq!(a, [3, 2, 1]);
		assert!(!next_permutation(&mut a));
	}
	{
		let mut counter = 0;
		let mut a = [1, 2, 3, 4];
		loop {
			counter += 1;
			if counter > 30 {
				panic!("It seems this is an infinite loop");
			}
			if !next_permutation(&mut a) {
				break;
			}
		}
		assert_eq!(counter, 24);
	}
}

#[test]
fn test_increase_vector() {
	{
		let mut a = [5];
		assert!(increase_vector(&mut a, 7));
		assert_eq!(a[0], 6);
		assert!(!increase_vector(&mut a, 6));
	}
	{
		let mut a = [8, 9];
		assert!(increase_vector(&mut a, 9));
		assert_eq!(a, [9, 0]);
		a[1] = 8;
		assert!(increase_vector(&mut a, 9));
		assert_eq!(a, [9, 9]);
		assert!(!increase_vector(&mut a, 9));
	}
}
