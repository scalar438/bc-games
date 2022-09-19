use std::cmp::Ord;

pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
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
	true
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_permutation1() {
		let mut a = [1];
		assert!(!next_permutation(&mut a));
	}

	#[test]
	fn test_permutation2() {
		let mut a = [1, 2];
		assert!(next_permutation(&mut a));
		assert_eq!(a, [2, 1]);
		assert!(!next_permutation(&mut a));
	}

	#[test]
	fn test_permutation3() {
		let mut a = [1, 1];
		assert!(!next_permutation(&mut a));
	}

	#[test]
	fn test_permutation4() {
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

	#[test]
	fn test_permutation5() {
		let mut a = [1, 2, 2];
		assert!(next_permutation(&mut a));
		assert_eq!(a, [2, 1, 2]);

		assert!(next_permutation(&mut a));
		assert_eq!(a, [2, 2, 1]);

		assert!(!next_permutation(&mut a));
	}

	#[test]
	fn test_permutation6() {
		let mut a = [1, 1, 2];
		assert!(next_permutation(&mut a));
		assert_eq!(a, [1, 2, 1]);

		assert!(next_permutation(&mut a));
		assert_eq!(a, [2, 1, 1]);

		assert!(!next_permutation(&mut a));
	}

	#[test]
	fn test_permutation7() {
		let mut a = [1, 1, 1];
		assert!(!next_permutation(&mut a));
	}
}
