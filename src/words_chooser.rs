use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum CharResult {
	NotPresented,
	PartialMatch,
	FullMatch,
}

#[derive(PartialEq, Debug)]
enum ChoiseState {
	ReadyToMakeGuess,
	WaitForRespond,
	NoMoreWords,
}

struct WordsContainer {
	vocabulary: Vec<String>,
	candidate_words: Vec<String>,
}

pub struct WordsChooser {
	words_container: WordsContainer,
	state: ChoiseState,
	word_len: u32,
	last_word: Option<String>,
}

impl WordsChooser {
	pub fn new<T: AsRef<str>>(all_words: &mut dyn Iterator<Item = &T>) -> WordsChooser {
		WordsChooser {
			words_container: WordsContainer::new(all_words),
			state: ChoiseState::ReadyToMakeGuess,
			word_len: 5, // TODO: make a right value
			last_word: None,
		}
	}

	pub fn make_guess(&mut self) -> Option<&str> {
		if self.state != ChoiseState::ReadyToMakeGuess {
			panic!("Cannot make guess, invalid state")
		}
		if self.words_container.candidate_words.is_empty() {
			self.state = ChoiseState::NoMoreWords;
			return None;
		}
		let mut max_val = u32::MAX;
		let mut res_attempt_word = "";
		let total_answers_count = {
			let mut d = 1;
			for _ in 0..self.word_len {
				d *= 3;
			}
			d
		};
		let mut vec_variants = Vec::new();
		vec_variants.resize(total_answers_count, 0);

		let mut answers_vec = Vec::new();

		for attempt_word in self.words_container.construct_list_of_words() {
			for v in vec_variants.iter_mut() {
				*v = 0
			}
			for hidden_word in self.words_container.candidate_words.iter() {
				calc_all_answers(attempt_word, hidden_word, &mut answers_vec);
				for res in answers_vec.iter() {
					vec_variants[*res as usize] += 1;
				}
			}
			let tmp_val = vec_variants.iter().max();
			if let Some(tmp_val) = tmp_val {
				if tmp_val < &max_val {
					max_val = *tmp_val;
					res_attempt_word = attempt_word;
				}
			}
		}

		self.state = ChoiseState::WaitForRespond;
		self.last_word = Some(res_attempt_word.to_owned());
		Some(res_attempt_word)
	}

	pub fn respond_to_guess(&mut self, respond: &[CharResult]) {
		if self.state != ChoiseState::WaitForRespond {
			panic!("Unexpected state: {:?}", self.state);
		}
		let respond = convert_res(respond);
		let mut tmp = Vec::new();
		let last_word = self.last_word.as_ref().unwrap();
		self.words_container.candidate_words.retain(|x| {
			calc_all_answers(&last_word, x, &mut tmp);
			tmp.contains(&respond)
		});

		self.state = ChoiseState::ReadyToMakeGuess;
	}
}

impl WordsContainer {
	fn new<T: AsRef<str>>(all_words: &mut dyn Iterator<Item = &T>) -> Self {
		let vocabulary: Vec<_> = all_words.map(|x| x.as_ref().to_owned()).collect();
		let candidate_words = vocabulary.clone();
		Self {
			vocabulary,
			candidate_words,
		}
	}

	fn construct_list_of_words(&self) -> Vec<&str> {
		if self.candidate_words.is_empty() {
			return Vec::new();
		}
		let candidate_strings: std::collections::HashSet<_> =
			self.candidate_words.iter().map(String::as_str).collect();

		let mut res: Vec<_> = self.vocabulary.iter().map(String::as_str).collect();

		let mut x = 0;
		let n = res.len();
		let mut y = n;

		while x < y {
			while x != n && candidate_strings.contains(res[x]) {
				x += 1;
			}
			while y != 0 && !candidate_strings.contains(res[y - 1]) {
				y -= 1;
			}
			if x < y {
				res.swap(x, y - 1);
				x += 1;
				y -= 1;
			}
		}

		res
	}
}

// Try to increase lexicographically the given array with following conditions:
// 1) Elements in the array are distinct and sorted
// 2) Elements in the array are less or equal max_val
// Precondition: array must be sorted before
// Return true if we have increased the array, otherwise return false
fn try_increase(arr: &mut [usize], mut max_val: usize) -> bool {
	let mut i = arr.len();
	loop {
		if i == 0 {
			return false;
		}
		i -= 1;
		if arr[i] == max_val {
			max_val -= 1;
		} else {
			arr[i] += 1;
			for i in i + 1..arr.len() {
				arr[i] = arr[i - 1] + 1;
			}
			return true;
		}
	}
}

#[test]
fn test_increase_1() {
	let mut arr = [1, 2, 3];
	assert!(try_increase(&mut arr, 4));
	assert_eq!(arr, [1, 2, 4]);
}

#[test]
fn test_increase_2() {
	let mut arr = [1, 2, 3];
	assert!(!try_increase(&mut arr, 3));
}

#[test]
fn test_increase_3() {
	let mut arr = [1, 3, 4];
	assert!(try_increase(&mut arr, 4));
	assert_eq!(arr, [2, 3, 4]);
	assert!(!try_increase(&mut arr, 4));
}

fn convert_res(arg: &[CharResult]) -> u32 {
	let mut res = 0;
	let mut delta = 1;
	for c in arg {
		match c {
			CharResult::FullMatch => res += delta + delta,
			CharResult::PartialMatch => res += delta,
			CharResult::NotPresented => {} // Do nothing
		}
		delta *= 3;
	}
	res
}

// Calculates all possible answers for the given hidden_word if we try an attempt_word as an attempt
// The main reason why we need to return vector instead of only one result is a letter repetitions
// For example, if the hidden word is "abba", and the attempt word is "baaa",
// the possible answers are "1102" and "1012" ("0" - NotPresented, "1" - "PartialMatch", "2" - "FullMatch")
// We match either second or third letter to the first "a" of hidden word
fn calc_all_answers(attempt_word: &str, hidden_word: &str, res: &mut Vec<u32>) {
	let len = attempt_word.chars().count();
	if len != hidden_word.chars().count() {
		panic!("Cannot compare strings with different lenght")
	}

	let make_hash = |x: &str| {
		let mut h = HashMap::new();
		for (i, c) in x.chars().enumerate() {
			h.entry(c).or_insert(Vec::new()).push(i);
		}
		h
	};
	let pow3 = {
		let mut p = Vec::new();
		let mut d = 1;
		for _ in 0..len {
			p.push(d);
			d *= 3;
		}
		p
	};

	res.clear();
	res.push(0);

	let attempt_hash = make_hash(attempt_word);
	let mut hidden_hash = make_hash(hidden_word);

	for (attempt_char, mut attempt_pos) in attempt_hash {
		if let Some(mut hidden_pos) = hidden_hash.remove(&attempt_char) {
			let old_attempt_pos_len = attempt_pos.len();
			let old_hidden_pos_len = std::cmp::min(hidden_pos.len(), old_attempt_pos_len);

			// We should remove positions that are also presented in the hidden_pos
			let mut new_attempt_pos = Vec::new();

			// Both vectors are sorted, so we can compare elements one-by-one by moving pointers and ignoring equal items
			// "Pointer" here is just the last element in the vector
			loop {
				match (attempt_pos.last(), hidden_pos.last()) {
					(Some(a), Some(h)) => {
						let va = *a;
						let vh = *h;
						if va <= vh {
							hidden_pos.pop();
						} else {
							new_attempt_pos.push(va);
						}
						if va >= vh {
							attempt_pos.pop();
						}
						if va == vh {
							// If we are here, characters at position va are equal
							for r_v in res.iter_mut() {
								*r_v += pow3[va] * 2;
							}
						}
					}
					(_, _) => {
						break;
					}
				}
			}

			new_attempt_pos.append(&mut attempt_pos);
			attempt_pos = new_attempt_pos;

			let num_of_matched = old_attempt_pos_len - attempt_pos.len();
			let num_of_hidden = old_hidden_pos_len - num_of_matched;

			if attempt_pos.len() <= num_of_hidden {
				for one_res in res.iter_mut() {
					for p in attempt_pos.iter() {
						*one_res += pow3[*p];
					}
				}
			} else {
				let mut tmp_res = Vec::new();
				let mut yellow_positions: Vec<_> = (0..num_of_hidden).collect();

				loop {
					for prev in res.iter() {
						let mut np = *prev;
						for v in yellow_positions.iter() {
							np += pow3[attempt_pos[*v]];
						}
						tmp_res.push(np);
					}
					if !try_increase(&mut yellow_positions, attempt_pos.len() - 1) {
						break;
					}
				}
				*res = tmp_res;
			}
		}
	}
}

#[cfg(test)]
mod test {

	use super::*;

	fn calc_all_answers_test(attempt_word: &str, hidden_word: &str) -> Vec<u32> {
		let mut res = Vec::new();
		calc_all_answers(attempt_word, hidden_word, &mut res);
		res
	}

	#[test]
	fn test_calc_matching_1() {
		let s1 = "abcd";
		let s2 = "xyzw";
		assert_eq!(
			calc_all_answers_test(s1, s2),
			[convert_res(&[CharResult::NotPresented; 4])]
		)
	}

	#[test]
	fn test_calc_matching_2() {
		let s1 = "abcd";
		let s2 = "abcd";
		assert_eq!(
			calc_all_answers_test(s1, s2),
			[convert_res(&[CharResult::FullMatch; 4])]
		)
	}

	#[test]
	fn test_calc_matching_3() {
		let s1 = "abcd";
		let s2 = "dcba";
		assert_eq!(
			calc_all_answers_test(s1, s2),
			[convert_res(&[CharResult::PartialMatch; 4])]
		)
	}

	#[test]
	fn test_calc_matching_4() {
		let s1 = "abcd";
		let s2 = "acba";
		assert_eq!(
			calc_all_answers_test(s1, s2),
			[convert_res(&[
				CharResult::FullMatch,
				CharResult::PartialMatch,
				CharResult::PartialMatch,
				CharResult::NotPresented
			])]
		)
	}

	#[test]
	fn test_calc_matching_5() {
		let s1 = "dbca";
		let s2 = "acba";
		assert_eq!(
			calc_all_answers_test(s1, s2),
			[convert_res(&[
				CharResult::NotPresented,
				CharResult::PartialMatch,
				CharResult::PartialMatch,
				CharResult::FullMatch
			])]
		)
	}

	#[test]
	fn test_calc_matching_6() {
		let s1 = "abba";
		let s2 = "babb";
		let mut r = calc_all_answers_test(s1, s2);
		assert_eq!(r.len(), 2);
		r.sort();
		let r = r;

		let f = |&x| match x {
			0 => CharResult::NotPresented,
			1 => CharResult::PartialMatch,
			2 => CharResult::FullMatch,
			_ => panic!("Invalid argument: {}", x),
		};

		let res_expected = vec![
			[0, 1, 2, 1].iter().map(f).collect::<Vec<_>>(),
			[1, 1, 2, 0].iter().map(f).collect::<Vec<_>>(),
		];
		let mut res_expected: Vec<_> = res_expected
			.iter()
			.map(|x| convert_res(x.as_slice()))
			.collect();
		res_expected.sort();
		assert_eq!(r, res_expected);
	}

	#[test]
	fn test_calc_matching_7() {
		// Two of "a"-s and one of "b" is PartialMatch
		let s1 = "ddaaabb";
		let s2 = "aabcccc";

		let res_expected = vec![
			[0, 0, 1, 1, 0, 1, 0],
			[0, 0, 1, 0, 1, 1, 0],
			[0, 0, 0, 1, 1, 1, 0],
			[0, 0, 1, 1, 0, 0, 1],
			[0, 0, 1, 0, 1, 0, 1],
			[0, 0, 0, 1, 1, 0, 1],
		];
		let mut res_expected: Vec<_> = res_expected
			.into_iter()
			.map(|arr| {
				arr.iter()
					.map(|x| match x {
						0 => CharResult::NotPresented,
						1 => CharResult::PartialMatch,
						2 => CharResult::FullMatch,
						_ => panic!("Invalid argument: {}", x),
					})
					.collect::<Vec<_>>()
			})
			.map(|x| convert_res(&x))
			.collect();
		res_expected.sort();

		let mut r = calc_all_answers_test(s1, s2);
		r.sort();
		assert_eq!(r, res_expected);
	}
}

#[test]
fn test_no_choise() {
	let vocabulary: Vec<_> = ["abc", "abd", "bad"]
		.iter()
		.map(|x| x.to_string())
		.collect();
	let candidate_words = vocabulary.clone();
	let mut w = WordsChooser {
		words_container: WordsContainer {
			vocabulary,
			candidate_words,
		},
		state: ChoiseState::ReadyToMakeGuess,
		word_len: 3,
		last_word: None,
	};

	assert!(w.make_guess().is_some());
	assert_eq!(w.state, ChoiseState::WaitForRespond);
	w.respond_to_guess(&[
		CharResult::NotPresented,
		CharResult::NotPresented,
		CharResult::NotPresented,
	]);

	assert!(w.words_container.candidate_words.is_empty());
	assert!(w.make_guess().is_none());
}

#[test]
fn test_one_choise() {
	let vocabulary: Vec<_> = ["abc", "abd"].iter().map(|x| x.to_string()).collect();
	let candidate_words = vocabulary.clone();
	let mut w = WordsChooser {
		words_container: WordsContainer {
			vocabulary,
			candidate_words,
		},
		state: ChoiseState::ReadyToMakeGuess,
		word_len: 3,
		last_word: None,
	};

	let attempt1 = w.make_guess().unwrap().to_owned();
	assert_eq!(w.last_word.as_ref().unwrap(), &attempt1);
	assert!(attempt1 == "abc" || attempt1 == "abd");
	w.respond_to_guess(&[
		CharResult::FullMatch,
		CharResult::FullMatch,
		CharResult::NotPresented,
	]);
	assert_eq!(w.words_container.candidate_words.len(), 1);
	let attempt2 = w.make_guess().unwrap();
	assert_ne!(attempt1, attempt2);
}

// In this test is the most reasonable choise as the first attempt is "abc" or "cbg"
// If we chose "bde", one of the possible answers is "100" is matched with two words - "fcb" and "abc",
// so we have to guess between them if we get this answer
// By the same reason, the word "fcb" and "011" as answer tells us the possible word either "abc" or "cbg"
// For the word "abc" we have three possbile answers (in assumption that our attempt isn't correct):
//    "010" tells as that word is "bde",
//    "011" - "fcb",
//    "021" - "cbg",
// Because we have no possible answers with more than one words, it is better than previous ones
// The "cbg" is a good choise too, because:
//    "010" - "bde"
//    "110" - "fcb"
//    "120" - "abc"
#[test]
fn test_smartest_choise() {
	let vocabulary: Vec<_> = ["bde", "fcb", "abc", "cbg"]
		.iter()
		.map(|x| x.to_string())
		.collect();
	let candidate_words = vocabulary.clone();
	let mut w = WordsChooser {
		words_container: WordsContainer {
			vocabulary,
			candidate_words,
		},
		state: ChoiseState::ReadyToMakeGuess,
		word_len: 3,
		last_word: None,
	};

	let attempt1 = w.make_guess().unwrap();
	// Check for the best options
	dbg!(&attempt1);
	assert!(attempt1 == "abc" || attempt1 == "cbg");

	// Let's assume we picked "bde". In this case the answer is "010" regardles of an attempt
	w.respond_to_guess(&[
		CharResult::NotPresented,
		CharResult::PartialMatch,
		CharResult::NotPresented,
	]);

	assert_eq!(w.words_container.candidate_words.len(), 1);
	assert_eq!(w.make_guess().unwrap(), "bde");
}
