use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Default)]
pub struct Number {
	data: Vec<u8>,
}

impl Display for Number {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let mut s = String::new();
		for b in self.data.iter() {
			match b {
				0..=9 => s.push((b + ('0' as u8)) as char),
				10..MAX_BASE => s.push((b - 10 + ('A' as u8)) as char),
				_ => unreachable!(),
			}
		}

		f.write_str(&s)
	}
}

impl<T> From<T> for Number
where
	T: AsRef<str>,
{
	fn from(value: T) -> Self {
		let data: Vec<_> = value
			.as_ref()
			.chars()
			.map(|c| match c {
				'0'..='9' => (c as u8) - ('0' as u8),
				'A'..='Z' => (c as u8) - 10 - ('A' as u8),
				_ => panic!("Unknown character"),
			})
			.collect();
		Self { data }
	}
}

#[derive(Clone, Copy)]
pub struct GameParams {
	number_len: u8,
	has_repetitions: bool,
	base: u8,
}

const MAX_BASE: u8 = 36;

impl GameParams {
	pub fn new(number_len: u8) -> Self {
		if number_len > MAX_BASE {
			panic!("number_len is too large");
		}
		Self {
			number_len,
			has_repetitions: false,
			base: 10,
		}
	}

	pub fn number_len(&self) -> u8 {
		self.number_len
	}

	pub fn with_repetitions(mut self, r: bool) -> Self {
		self.has_repetitions = r;
		self
	}

	pub fn with_base(mut self, base: u8) -> Self {
		if base > MAX_BASE {
			panic!("Number of digits can't be more than {MAX_BASE}");
		}
		self.base = base;
		self
	}

	pub fn to_number_checked(&self, s: &str) -> Option<Number> {
		let mut res = Vec::new();
		let mut digit_presented = vec![false; MAX_BASE as usize];
		for c in s.chars() {
			match self.to_u8(c) {
				Ok(v) => {
					if !self.has_repetitions && digit_presented[v as usize] {
						// It is a repetition
						return None;
					}
					digit_presented[v as usize] = true;
					res.push(v);
				}

				Err(_) => return None,
			}
		}
		if res.len() as u8 != self.number_len {
			None
		} else {
			Some(Number { data: res })
		}
	}

	fn to_string_checked(&self, num: &Number) -> Option<String> {
		let mut res = String::new();
		let mut digit_presented = vec![false; MAX_BASE as usize];

		if num.data.len() as u8 != self.number_len {
			return None;
		}
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
		if b >= self.base {
			return Err(format!("value {b} is too large"));
		}
		match b {
			0..=9 => Ok((b + ('0' as u8)) as char),
			10..=MAX_BASE => Ok((b - 10 + ('A' as u8)) as char),
			_ => unreachable!(),
		}
	}

	fn to_u8(&self, c: char) -> Result<u8, String> {
		let v = match c {
			'0'..='9' => ((c as u8) - ('0' as u8)) as u8,
			'A'..='Z' => ((c as u8) - ('A' as u8) + 10) as u8,
			_ => return Err(format!("Char {c} isn't in valid range")),
		};
		if v >= self.base {
			Err(format!("Char {c} represents too large digit"))
		} else {
			Ok(v)
		}
	}

	pub fn calc_bc(&self, a: &Number, b: &Number) -> (u8, u8) {
		calc_bc_with_size(a, b, self.base)
	}
}

fn get_numbers_iter_ref(g: &GameParams) -> Box<dyn RefIter<Item = Number>> {
	if !g.has_repetitions {
		Box::new(NumbersWithoutRepetitions {
			cur_number: None,
			used_digits: Vec::new(),
			number_len: g.number_len,
			total_digits_number: g.base,
		})
	} else {
		Box::new(NumbersWithRepetitions {
			cur_number: None,
			number_len: g.number_len,
			total_digits_number: g.base,
		})
	}
}

pub fn get_numbers_iter(g: &GameParams) -> Box<dyn Iterator<Item = Number>> {
	if !g.has_repetitions {
		Box::new(NumbersWithoutRepetitions {
			cur_number: None,
			used_digits: Vec::new(),
			number_len: g.number_len,
			total_digits_number: g.base,
		})
	} else {
		Box::new(NumbersWithRepetitions {
			cur_number: None,
			number_len: g.number_len,
			total_digits_number: g.base,
		})
	}
}

pub trait RefIter {
	type Item;

	fn next(&mut self) -> Option<&Self::Item>;
}

struct NumbersWithoutRepetitions {
	cur_number: Option<Number>,
	used_digits: Vec<bool>,
	number_len: u8,
	total_digits_number: u8,
}

impl Iterator for NumbersWithoutRepetitions {
	type Item = Number;

	fn next(&mut self) -> Option<Self::Item> {
		let res = (self as &mut dyn RefIter<Item = Self::Item>).next();
		res.map(|x| x.clone())
	}
}

impl RefIter for NumbersWithoutRepetitions {
	type Item = Number;

	fn next(&mut self) -> Option<&Self::Item> {
		match &mut self.cur_number {
			Some(num) => {
				if !next_permutation(&mut num.data) {
					if next_permutation(&mut self.used_digits) {
						num.data = self
							.used_digits
							.iter()
							.enumerate()
							.filter_map(|(i, v)| if *v { Some(i as u8) } else { None })
							.collect();
					} else {
						self.cur_number = None;
					}
				}
			}

			None => {
				self.used_digits = std::iter::repeat(false)
					.take((self.total_digits_number - self.number_len) as usize)
					.chain(std::iter::repeat(true).take(self.number_len as usize))
					.collect();

				debug_assert_eq!(self.used_digits.len() as u8, self.total_digits_number);
				self.cur_number = Some(Number {
					data: self
						.used_digits
						.iter()
						.enumerate()
						.filter_map(|(i, v)| if *v { Some(i as u8) } else { None })
						.collect(),
				});
			}
		}

		match &self.cur_number {
			Some(x) => Some(x),
			None => None,
		}
	}
}

struct NumbersWithRepetitions {
	cur_number: Option<Number>,
	number_len: u8,
	total_digits_number: u8,
}

impl Iterator for NumbersWithRepetitions {
	type Item = Number;

	fn next(&mut self) -> Option<Self::Item> {
		let res = (self as &mut dyn RefIter<Item = Self::Item>).next();
		res.map(|x| x.clone())
	}
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

pub fn calc_bc_with_size(a: &Number, b: &Number, base: u8) -> (u8, u8) {
	let mut count_a = vec![0; base as usize];
	let mut count_b = count_a.clone();
	let mut bulls: i32 = 0;
	for (digit_a, digit_b) in a.data.iter().zip(b.data.iter()) {
		if *digit_a == *digit_b {
			bulls += 1;
		}
		count_a[*digit_a as usize] += 1;
		count_b[*digit_b as usize] += 1;
	}
	let mut cows: i32 = -bulls;
	for (c_a, c_b) in count_a.into_iter().zip(count_b.into_iter()) {
		cows += i32::min(c_a, c_b);
	}
	(bulls as u8, cows as u8)
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
	fn test_permutation_rep() {
		{
			let mut a = [0, 0, 1];
			assert!(next_permutation(&mut a));
			assert_eq!(a, [0, 1, 0]);
			assert!(next_permutation(&mut a));
			assert_eq!(a, [1, 0, 0]);
			assert!(!next_permutation(&mut a));
		}
		{
			let mut a = [0, 0, 1, 1];
			assert!(next_permutation(&mut a));
			assert_eq!(a, [0, 1, 0, 1]);
			assert!(next_permutation(&mut a));
			assert_eq!(a, [0, 1, 1, 0]);
			assert!(next_permutation(&mut a));
			assert_eq!(a, [1, 0, 0, 1]);
			assert!(next_permutation(&mut a));
			assert_eq!(a, [1, 0, 1, 0]);
			assert!(next_permutation(&mut a));
			assert_eq!(a, [1, 1, 0, 0]);
			assert!(!next_permutation(&mut a));
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
		let g = GameParams::new(5).with_base(6);
		assert_eq!(g.to_char(0).unwrap(), '0');
		assert_eq!(g.to_char(5).unwrap(), '5');
		assert!(g.to_char(6).is_err());
		assert!(g.to_char(13).is_err());
		assert!(g.to_char(40).is_err());
	}

	#[test]
	fn test_to_char_10() {
		let g = GameParams::new(5).with_base(10);
		assert_eq!(g.to_char(0).unwrap(), '0');
		assert_eq!(g.to_char(5).unwrap(), '5');
		assert_eq!(g.to_char(9).unwrap(), '9');
		assert!(g.to_char(10).is_err());
		assert!(g.to_char(13).is_err());
		assert!(g.to_char(40).is_err());
	}

	#[test]
	fn test_to_char_17() {
		let g = GameParams::new(5).with_base(17);
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
		let g = GameParams::new(2).with_base(6);
		assert_eq!(g.to_u8('0').unwrap(), 0);
		assert_eq!(g.to_u8('5').unwrap(), 5);
		assert!(g.to_u8('6').is_err());
		assert!(g.to_u8('D').is_err());
		assert!(g.to_u8('&').is_err());
	}

	#[test]
	fn test_to_u8_10() {
		let g = GameParams::new(2).with_base(10);
		assert_eq!(g.to_u8('0').unwrap(), 0);
		assert_eq!(g.to_u8('5').unwrap(), 5);
		assert_eq!(g.to_u8('9').unwrap(), 9);
		assert!(g.to_u8('A').is_err());
		assert!(g.to_u8('D').is_err());
		assert!(g.to_u8('&').is_err());
	}

	#[test]
	fn test_to_u8_13() {
		let g = GameParams::new(2).with_base(13);
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
			g.to_string_checked(&gen_number(&[6, 7, 4, 0])).unwrap(),
			"6740"
		);

		assert!(g.to_string_checked(&gen_number(&[7, 7, 4, 0])).is_none());
		assert!(g.to_string_checked(&gen_number(&[7, 10, 4, 0])).is_none());

		let g = g.with_repetitions(true);
		assert_eq!(
			g.to_string_checked(&gen_number(&[6, 7, 4, 0])).unwrap(),
			"6740"
		);
		assert_eq!(
			g.to_string_checked(&gen_number(&[7, 7, 4, 0])).unwrap(),
			"7740"
		);
		assert!(g.to_string_checked(&gen_number(&[7, 10, 4, 0])).is_none());
	}

	#[test]
	fn test_calc_bc() {
		assert_eq!(
			calc_bc_with_size(&Number::from("0123"), &Number::from("0432"), 5),
			(1, 2)
		);
		assert_eq!(
			calc_bc_with_size(&Number::from("1234"), &Number::from("5678"), 10),
			(0, 0)
		);
		assert_eq!(
			calc_bc_with_size(&Number::from("12304"), &Number::from("43210"), 10),
			(0, 5)
		);
		assert_eq!(
			calc_bc_with_size(&Number::from("123456"), &Number::from("123456"), 10),
			(6, 0)
		);
		assert_eq!(
			calc_bc_with_size(&Number::from("1234"), &Number::from("7893"), 10),
			(0, 1)
		);
	}

	#[test]
	fn test_gen_numbers() {
		{
			let g = GameParams::new(1).with_base(5);
			let it = get_numbers_iter(&g);
			let mut v = Vec::new();
			for x in it {
				v.push(x.to_string());
			}
			v.sort();
			assert_eq!(v, ["0", "1", "2", "3", "4"]);
		}

		{
			let g = GameParams::new(2).with_base(3);
			let it = get_numbers_iter(&g);
			let mut v = Vec::new();
			for x in it {
				v.push(x.to_string());
			}
			v.sort();

			let mut expected = ["01", "10", "21", "12", "20", "02"];
			expected.sort();
			assert_eq!(v, expected);
		}

		{
			let g = GameParams::new(3).with_base(3);
			let it = get_numbers_iter(&g);
			let mut v = Vec::new();
			for x in it {
				v.push(x.to_string());
			}
			v.sort();

			let mut expected = ["012", "021", "102", "120", "210", "201"];
			expected.sort();
			assert_eq!(v, expected);
		}
	}
}
