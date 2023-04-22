use std::io::{BufRead, Write};

pub struct WordsDb {
	words: Vec<String>,
	db_filename: std::path::PathBuf,
	new_words: Vec<String>,
	word_current_len: usize,
}

impl WordsDb {
	pub fn new(path: &std::path::Path, word_len: usize) -> std::io::Result<Self> {
		match std::fs::File::open(path) {
			Ok(mut f) => Self::new_from_file(&mut f, path, word_len),
			Err(_) => Ok(Self {
				words: Vec::new(),
				new_words: Vec::new(),
				db_filename: path.to_path_buf(),
				word_current_len: word_len,
			}),
		}
	}

	fn new_from_file(
		f: &mut dyn std::io::Read,
		path: &std::path::Path,
		word_len: usize,
	) -> std::io::Result<Self> {
		let mut words = Vec::new();
		for line in std::io::BufReader::new(f).lines() {
			let line = line?;
			let line = line.trim();
			if line.chars().count() == word_len {
				words.push(line.to_string());
			}
		}
		Ok(Self {
			words,
			db_filename: path.to_path_buf(),
			new_words: Vec::new(),
			word_current_len: word_len,
		})
	}

	pub fn add_word(&mut self, word: &str) {
		let word = word.trim();
		if word.chars().count() != self.word_current_len {
			panic!("Invalid word len");
		}
		self.new_words.push(word.to_lowercase());
	}

	pub fn words_iter(&self) -> impl Iterator<Item = &String> + '_ {
		self.words.iter().chain(self.new_words.iter())
	}

	pub fn flush(&mut self) -> std::io::Result<()> {
		let mut all_words: Vec<_> = self
			.words
			.iter()
			.chain(self.new_words.iter())
			.map(|x| x.clone())
			.collect();
		if let Ok(f) = std::fs::File::open(&self.db_filename) {
			for line in std::io::BufReader::new(f).lines() {
				let line = line?;
				let line = line.trim();
				all_words.push(line.to_owned());
			}
		}

		all_words.sort_by(|a, b| {
			let r1 = a.len().cmp(&b.len());
			if !r1.is_eq() {
				return r1;
			}
			return a.cmp(&b);
		});
		all_words.dedup();
		{
			let mut f = std::fs::File::create(&self.db_filename)?;
			f.write(all_words.join("\n").as_bytes())?;
		}

		self.new_words = Vec::new();
		self.words = all_words;

		return Ok(());
	}
}

impl Drop for WordsDb {
	fn drop(&mut self) {
		if self.new_words.is_empty() {
			return;
		}

		if self.flush().is_err() {
			eprintln!("Unexpected error during saving the word database file");
		}
	}
}

#[cfg(test)]
mod test {
	use std::io::Seek;

	use super::*;
	use tempfile;

	struct FileDeleter<'a> {
		path: std::path::PathBuf,
		res: &'a mut bool,
	}

	impl<'a> Drop for FileDeleter<'a> {
		fn drop(&mut self) {
			let r = std::fs::remove_file(&self.path);
			*self.res = r.is_err();
			if r.is_err() {
				eprintln!(
					"File delete error, filename: {:?}, err: {:?}",
					self.path,
					r.unwrap_err()
				)
			}
		}
	}

	#[test]
	fn test_init() {
		let fname = std::path::Path::new("./test_create_db");
		if fname.exists() {
			panic!(
				"The file/folder {:?} is exists before running tests!",
				fname
			);
		}

		let mut file_delete_failed = false;
		{
			let _file_deleter = FileDeleter {
				path: fname.to_path_buf(),
				res: &mut file_delete_failed,
			};
			{
				let db = WordsDb::new(fname, 5);

				let db = db.unwrap();
				assert!(db.words.is_empty());
			}
		}
	}

	#[test]
	fn test_read() {
		let mut f = tempfile::spooled_tempfile(100000);

		let words = ["bar", "baz", "foo"];
		f.write(words.join("\n").as_bytes()).unwrap();
		f.seek(std::io::SeekFrom::Start(0)).unwrap();

		{
			let db = WordsDb::new_from_file(&mut f, std::path::Path::new(""), 3);

			let db = db.unwrap();
			assert_eq!(db.words, words);
		}
	}

	#[test]
	fn test_read_and_write() {
		let fname = std::path::Path::new("./test_read_and_write_db");
		if fname.exists() {
			panic!(
				"The file/folder {:?} is exists before running tests!",
				fname
			);
		}

		let mut file_delete_failed = false;
		{
			let _file_deleter = FileDeleter {
				path: fname.to_path_buf(),
				res: &mut file_delete_failed,
			};
			let mut all_words = Vec::new();

			{
				let db = WordsDb::new(fname, 3);

				let mut db = db.unwrap();
				let words = ["foo", "bar", "baz"];
				for word in words {
					db.add_word(word);
					all_words.push(word.to_owned());
				}
			}
			all_words.sort();

			{
				let db = WordsDb::new(fname, 3);

				let mut db = db.unwrap();
				assert_eq!(db.words, all_words);
				db.add_word("qwe");
				all_words.push("qwe".to_owned());
			}
			all_words.sort();

			{
				let db = WordsDb::new(fname, 3);

				let db = db.unwrap();
				assert_eq!(db.words, all_words);
			}
		}
		assert!(!file_delete_failed);
	}

	#[test]
	fn test_multiple_len() {
		let fname = std::path::Path::new("./test_multiple_len_db");
		if fname.exists() {
			panic!(
				"The file/folder {:?} is exists before running tests!",
				fname
			);
		}

		let mut file_delete_failed = false;
		{
			let _file_deleter = FileDeleter {
				path: fname.to_path_buf(),
				res: &mut file_delete_failed,
			};

			{
				let mut db = WordsDb::new(fname, 3).unwrap();

				let words = ["foo", "bar", "baz"];
				for word in words {
					db.add_word(word);
				}
			}

			{
				let mut db = WordsDb::new(fname, 4).unwrap();

				assert!(db.words.is_empty());
				db.add_word("abcd");
			}

			{
				let db = WordsDb::new(fname, 3).unwrap();

				assert_eq!(db.words, ["bar", "baz", "foo"]);
			}

			{
				let db = WordsDb::new(fname, 4).unwrap();

				assert_eq!(db.words, ["abcd"]);
			}
		}
		assert!(!file_delete_failed);
	}
}
