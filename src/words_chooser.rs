use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum CharResult {
	NotPresented,
	PartialMatch,
	FullMatch,
}

pub struct WordsChooser {
	vocabulary: Vec<String>,
	matched_words: Vec<String>,
	word_len: usize,
}

impl WordsChooser {
	pub fn new<T: AsRef<str>>(
		word_len: usize,
		all_words: &mut dyn Iterator<Item = &T>,
	) -> WordsChooser {
		WordsChooser {
			vocabulary: all_words.map(|x| x.as_ref().to_owned()).collect(),
			matched_words: Vec::new(),
			word_len,
		}
	}

	pub fn init(&mut self) {
		self.matched_words = self.vocabulary.clone();
		unimplemented!()
	}

	pub fn make_guess(&self) -> Option<&str> {
		if self.matched_words.len() == 0 {
			return None;
		}
		unimplemented!()
	}

	pub fn respond_to_guess(&mut self, respond: &[CharResult]) {
		unimplemented!()
	}

	pub fn add_word_to_list<T: AsRef<str>>(&mut self, word: T) {
		unimplemented!()
	}
}

// Calculates all possible answers for the given hidden_word if we try an attempt_word as an attempt
// The main reason why we need returning the vector instead of only one result is a letter repetitions
// For example, if the hidden word is "abba", and the attempt word is "baaa",
// the possible answers are "1102" and "1012" ("0" - NotPresented, "1" - "PartialMatch", "2" - "FullMatch")
// We match either second or third letter to the first "a" of hidden word
fn calc_all_answers(attempt_word: &str, hidden_word: &str) -> Vec<Vec<CharResult>> {
	if attempt_word.len() != hidden_word.len() {
		panic!("Cannot compare strings with different lenght")
	}

	let make_hash = |x: &str| {
		let mut h = HashMap::new();
		for (i, c) in x.chars().enumerate() {
			h.entry(c).or_insert(Vec::new()).push(i);
		}
		h
	};

	let mut res = vec![vec![CharResult::NotPresented; attempt_word.len()]];

	let attempt_hash = make_hash(attempt_word);
	let mut hidden_hash = make_hash(hidden_word);

	for (attempt_char, mut attempt_pos) in attempt_hash {
		if let Some(mut hidden_pos) = hidden_hash.remove(&attempt_char) {
			let old_attempt_pos_len = attempt_pos.len();
			let old_hidden_pos_len = std::cmp::min(hidden_pos.len(), old_attempt_pos_len);

			// We should remove positions that are also presented in the hidden_pos
			let mut new_attempt_pos = Vec::new();

			// Both vectors are sorted, so we can compare elements one-by-one by moving the pointers and ignoring equal items
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
							for vec_r in res.iter_mut() {
								vec_r[va] = CharResult::FullMatch;
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
						if one_res[*p] != CharResult::FullMatch {
							one_res[*p] = CharResult::PartialMatch;
						}
					}
				}
			} else {
				let end_index = res.len();
			}
		}
	}

	res
}

#[test]
fn test_calc_matching_1() {
	let s1 = "abcd";
	let s2 = "xyzw";
	assert_eq!(calc_all_answers(s1, s2), [[CharResult::NotPresented; 4]])
}

#[test]
fn test_calc_matching_2() {
	let s1 = "abcd";
	let s2 = "abcd";
	assert_eq!(calc_all_answers(s1, s2), [[CharResult::FullMatch; 4]])
}

#[test]
fn test_calc_matching_3() {
	let s1 = "abcd";
	let s2 = "dcba";
	assert_eq!(calc_all_answers(s1, s2), [[CharResult::PartialMatch; 4]])
}

#[test]
fn test_calc_matching_4() {
	let s1 = "abcd";
	let s2 = "acba";
	assert_eq!(
		calc_all_answers(s1, s2),
		[[
			CharResult::FullMatch,
			CharResult::PartialMatch,
			CharResult::PartialMatch,
			CharResult::NotPresented
		]]
	)
}

#[test]
fn test_calc_matching_5() {
	let s1 = "dbca";
	let s2 = "acba";
	assert_eq!(
		calc_all_answers(s1, s2),
		[[
			CharResult::NotPresented,
			CharResult::PartialMatch,
			CharResult::PartialMatch,
			CharResult::FullMatch
		]]
	)
}

#[test]
fn test_calc_matching_6() {
	let s1 = "abba";
	let s2 = "babb";
	let mut r = calc_all_answers(s1, s2);
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
		.collect();
	res_expected.sort();

	let mut r = calc_all_answers(s1, s2);
	r.sort();
	assert_eq!(r, res_expected);
}
