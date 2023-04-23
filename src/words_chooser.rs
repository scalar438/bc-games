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
	next_variants: HashMap<Vec<CharResult>, Vec<String>>,
	state: ChoiseState,
}

impl WordsChooser {
	pub fn new<T: AsRef<str>>(all_words: &mut dyn Iterator<Item = &T>) -> WordsChooser {
		WordsChooser {
			words_container: WordsContainer::new(all_words),
			next_variants: HashMap::new(),
			state: ChoiseState::ReadyToMakeGuess,
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
		let mut all_variants = std::collections::HashMap::new();
		let mut max_val = usize::MAX;
		let mut res_attempt_word = "";
		for attempt_word in self.words_container.construct_list_of_words() {
			let mut all_variants_tmp = std::collections::HashMap::new();
			let mut max_val_tmp = 0;
			for hidden_word in self.words_container.candidate_words.iter() {
				for res in calc_all_answers(attempt_word, hidden_word) {
					let v = all_variants_tmp.entry(res).or_insert(Vec::new());
					v.push(hidden_word);
					max_val_tmp = std::cmp::max(v.len(), max_val_tmp);
				}
			}
			if max_val_tmp < max_val {
				max_val = max_val_tmp;
				all_variants = all_variants_tmp;
				res_attempt_word = &(*attempt_word);
			}
		}

		self.next_variants = all_variants
			.drain()
			.map(|(k, v)| (k, v.into_iter().map(|x| x.clone()).collect()))
			.collect();

		self.state = ChoiseState::WaitForRespond;
		Some(res_attempt_word)
	}

	pub fn respond_to_guess(&mut self, respond: &[CharResult]) {
		if self.state != ChoiseState::WaitForRespond {
			panic!("Unexpected state: {:?}", self.state);
		}
		if let Some(next) = self.next_variants.remove(respond) {
			self.words_container.candidate_words = next;
		} else {
			self.words_container.candidate_words.clear();
		}
		self.next_variants.clear();
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

// Calculates all possible answers for the given hidden_word if we try an attempt_word as an attempt
// The main reason why we need to return vector instead of only one result is a letter repetitions
// For example, if the hidden word is "abba", and the attempt word is "baaa",
// the possible answers are "1102" and "1012" ("0" - NotPresented, "1" - "PartialMatch", "2" - "FullMatch")
// We match either second or third letter to the first "a" of hidden word
fn calc_all_answers(attempt_word: &str, hidden_word: &str) -> Vec<Vec<CharResult>> {
	if attempt_word.chars().count() != hidden_word.chars().count() {
		panic!("Cannot compare strings with different lenght")
	}

	let make_hash = |x: &str| {
		let mut h = HashMap::new();
		for (i, c) in x.chars().enumerate() {
			h.entry(c).or_insert(Vec::new()).push(i);
		}
		h
	};

	let mut res = vec![vec![CharResult::NotPresented; attempt_word.chars().count()]];

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
				let mut tmp_res = Vec::new();
				let mut yellow_positions: Vec<_> = (0..num_of_hidden).collect();

				loop {
					for prev in res.iter() {
						let mut np = prev.clone();
						for v in yellow_positions.iter() {
							np[attempt_pos[*v]] = CharResult::PartialMatch;
						}
						tmp_res.push(np);
					}
					if !try_increase(&mut yellow_positions, attempt_pos.len() - 1) {
						break;
					}
				}
				res = tmp_res;
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
		next_variants: HashMap::new(),
		state: ChoiseState::ReadyToMakeGuess,
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
		next_variants: HashMap::new(),
		state: ChoiseState::ReadyToMakeGuess,
	};

	let attempt1 = w.make_guess().unwrap().to_owned();
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

// In this test is most sensible choise as the first attempt is "abc" or "cbg"
// If we chose "bde", one of the possible answers is "100" is matched with two words - "fcb" and "abc",
// so we have to guess between them if we get this answer
// By the same reason, the word "fcb" and "011" as answer tells us the possible word either "abc" or "cbg"
// For the word "abc" we have three possbile answers (in assumption that our attempt isn't correct):
//    "010" tells as that word is "bde",
//    "011" - "fcb",
//    "021" - "cbg",
// Because we have no possible answers with more than one words, it is better than previous ones
// The "cbg" is a good choise too:
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
		next_variants: HashMap::new(),
		state: ChoiseState::ReadyToMakeGuess,
	};

	let attempt1 = w.make_guess().unwrap();
	// Check for the best options
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
