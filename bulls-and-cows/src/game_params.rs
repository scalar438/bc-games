use std::{
	fmt::{Display, Formatter},
	marker::PhantomData,
	mem,
};

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Number {
	data: Vec<u8>,
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let mut s = String::new();
		for b in self.data.iter() {
			match b {
				0..=9 => s.push((b + ('0' as u8)) as char),
				10..MAX_DIGITS_SET_SIZE => s.push((b - 10 + ('A' as u8)) as char),
				_ => unreachable!(),
			}
		}
		f.write_str(&s)
	}
}

pub trait RefIter {
	type Item;

	fn next(&mut self) -> Option<&Self::Item>;
}

pub struct GameParams {
	number_len: u8,
	has_repetitions: bool,
	total_digits_number: u8,
}

const MAX_DIGITS_SET_SIZE: u8 = 36;

impl GameParams {
	pub fn new(number_len: u8) -> Self {
		if number_len > MAX_DIGITS_SET_SIZE {
			panic!("num_of_digits too large");
		}
		Self {
			number_len,
			has_repetitions: false,
			total_digits_number: 10,
		}
	}

	pub fn with_repetitions(mut self, r: bool) -> Self {
		self.has_repetitions = r;
		self
	}

	pub fn with_total_digits_number(mut self, ds: u8) -> Self {
		if ds > MAX_DIGITS_SET_SIZE {
			panic!("Number of digits can't be more than {MAX_DIGITS_SET_SIZE}");
		}
		self.total_digits_number = ds;
		self
	}

	pub fn get_numbers_iter(&self) -> Box<dyn RefIter<Item = Number>> {
		if !self.has_repetitions {
			todo!()
		} else {
			Box::new(NumbersWithRepetitions {
				cur_number: None,
				number_len: self.number_len,
				total_digits_number: self.total_digits_number,
			})
		}
	}

	fn to_byte_vec_value(&self, s: &str) -> Option<Number> {
		let mut res = Vec::new();
		let mut digit_presented = vec![false; MAX_DIGITS_SET_SIZE as usize];
		for c in s.chars() {
			match self.to_u8(c) {
				Ok(v) => {
					if !self.has_repetitions && digit_presented[v as usize] {
						// It's a repetitions
						return None;
					}
					digit_presented[v as usize] = true;
					res.push(v);
				}

				Err(_) => return None,
			}
		}

		Some(Number { data: res })
	}

	fn to_string_value(&self, num: &Number) -> Option<String> {
		let mut res = String::new();
		let mut digit_presented = vec![false; MAX_DIGITS_SET_SIZE as usize];

		for b in num.data.iter() {
			if !self.has_repetitions && digit_presented[*b as usize] {
				return None;
			}
			digit_presented[*b as usize] = true;
			match self.to_char(*b) {
				Ok(c) => res.push(c),
				Err(_) => return None,
			}
		}

		Some(res)
	}

	fn to_char(&self, b: u8) -> Result<char, String> {
		if b >= self.total_digits_number {
			return Err(format!("value {b} is too large"));
		}
		match b {
			0..=9 => Ok((b + ('0' as u8)) as char),
			10..=MAX_DIGITS_SET_SIZE => Ok((b - 10 + ('A' as u8)) as char),
			_ => unreachable!(),
		}
	}

	fn to_u8(&self, c: char) -> Result<u8, String> {
		let v = match c {
			'0'..='9' => ((c as u8) - ('0' as u8)) as u8,
			'A'..='Z' => ((c as u8) - ('A' as u8) + 10) as u8,
			_ => return Err(format!("Char {c} isn't in valid range")),
		};
		if v >= self.total_digits_number {
			Err(format!("Char {c} represents too large digit"))
		} else {
			Ok(v)
		}
	}
}

struct NumbersWithoutRepetitions {
	cur_numbers: Vec<u8>,
}

impl Iterator for NumbersWithoutRepetitions {
	type Item = Number;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

struct NumbersWithRepetitions {
	cur_number: Option<Number>,
	number_len: u8,
	total_digits_number: u8,
}

impl RefIter for NumbersWithRepetitions {
	type Item = Number;

	fn next(&mut self) -> Option<&Self::Item> {
		match &mut self.cur_number {
			Some(num) => {
				if !increase_vector(&mut num.data, self.total_digits_number) {
					self.cur_number = None;
				}
			}

			None => {
				let v = std::iter::repeat(0)
					.take(self.number_len as usize)
					.collect();

				self.cur_number = Some(Number { data: v });
			}
		}

		match &self.cur_number {
			Some(x) => Some(x),
			None => None,
		}
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

fn increase_vector(a: &mut [u8], maxval: u8) -> bool {
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

#[cfg(test)]
mod test {

	use super::*;

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

	#[test]
	fn test_to_char_6() {
		let g = GameParams::new(5).with_total_digits_number(6);
		assert_eq!(g.to_char(0).unwrap(), '0');
		assert_eq!(g.to_char(5).unwrap(), '5');
		assert!(g.to_char(6).is_err());
		assert!(g.to_char(13).is_err());
		assert!(g.to_char(40).is_err());
	}

	#[test]
	fn test_to_char_10() {
		let g = GameParams::new(5).with_total_digits_number(10);
		assert_eq!(g.to_char(0).unwrap(), '0');
		assert_eq!(g.to_char(5).unwrap(), '5');
		assert_eq!(g.to_char(9).unwrap(), '9');
		assert!(g.to_char(10).is_err());
		assert!(g.to_char(13).is_err());
		assert!(g.to_char(40).is_err());
	}

	#[test]
	fn test_to_char_17() {
		let g = GameParams::new(5).with_total_digits_number(17);
		assert_eq!(g.to_char(0).unwrap(), '0');
		assert_eq!(g.to_char(5).unwrap(), '5');
		assert_eq!(g.to_char(9).unwrap(), '9');
		assert_eq!(g.to_char(13).unwrap(), 'D');
		assert_eq!(g.to_char(16).unwrap(), 'G');
		assert!(g.to_char(17).is_err());
		assert!(g.to_char(20).is_err());
		assert!(g.to_char(40).is_err());
	}

	#[test]
	fn test_to_u8_6() {
		let g = GameParams::new(2).with_total_digits_number(6);
		assert_eq!(g.to_u8('0').unwrap(), 0);
		assert_eq!(g.to_u8('5').unwrap(), 5);
		assert!(g.to_u8('6').is_err());
		assert!(g.to_u8('D').is_err());
		assert!(g.to_u8('&').is_err());
	}

	#[test]
	fn test_to_u8_10() {
		let g = GameParams::new(2).with_total_digits_number(10);
		assert_eq!(g.to_u8('0').unwrap(), 0);
		assert_eq!(g.to_u8('5').unwrap(), 5);
		assert_eq!(g.to_u8('9').unwrap(), 9);
		assert!(g.to_u8('A').is_err());
		assert!(g.to_u8('D').is_err());
		assert!(g.to_u8('&').is_err());
	}

	#[test]
	fn test_to_u8_13() {
		let g = GameParams::new(2).with_total_digits_number(13);
		assert_eq!(g.to_u8('0').unwrap(), 0);
		assert_eq!(g.to_u8('5').unwrap(), 5);
		assert_eq!(g.to_u8('9').unwrap(), 9);
		assert_eq!(g.to_u8('A').unwrap(), 10);
		assert_eq!(g.to_u8('C').unwrap(), 12);
		assert!(g.to_u8('D').is_err());
		assert!(g.to_u8('Q').is_err());
		assert!(g.to_u8('&').is_err());
	}

	fn gen_number(b: &[u8]) -> Number {
		Number {
			data: b.iter().map(|x| *x).collect(),
		}
	}

	#[test]
	fn test_to_string_value() {
		let g = GameParams::new(4);
		assert_eq!(
			g.to_string_value(&gen_number(&[6, 7, 4, 0])).unwrap(),
			"6740"
		);

		assert!(g.to_string_value(&gen_number(&[7, 7, 4, 0])).is_none());
		assert!(g.to_string_value(&gen_number(&[7, 10, 4, 0])).is_none());

		let g = g.with_repetitions(true);
		assert_eq!(
			g.to_string_value(&gen_number(&[6, 7, 4, 0])).unwrap(),
			"6740"
		);
		assert_eq!(
			g.to_string_value(&gen_number(&[7, 7, 4, 0])).unwrap(),
			"7740"
		);
		assert!(g.to_string_value(&gen_number(&[7, 10, 4, 0])).is_none());
	}
}
