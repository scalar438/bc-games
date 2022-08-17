use std::{
	cmp::Ordering,
	io::{BufRead, Read, Write},
};

pub struct WordsDb {
	words: Vec<String>,
	db_filename: std::path::PathBuf,
	new_words: Vec<String>,
}

impl WordsDb {
	pub fn new(path: &std::path::Path) -> std::io::Result<Self> {
		let mut f = std::fs::File::open(path);

		let words;
		if f.is_err() {
			// Just for creation
			f = std::fs::File::create(path);

			words = Vec::new();
		} else {
			words = std::io::BufReader::new(f?)
				.lines()
				.map(|x| x.unwrap())
				.collect();
		}

		Ok(Self {
			words,
			new_words: Vec::new(),
			db_filename: path.to_path_buf(),
		})
	}

	pub fn add_word(&mut self, word: &str) {
		self.new_words.push(word.to_owned());
	}

	fn drop_with_checks(&mut self) -> std::io::Result<()> {
		let mut f = std::fs::File::create(&self.db_filename)?;

		for line in &self.words {
			f.write(line.as_bytes())?;
			f.write(b"\n")?;
		}

		return Ok(());
	}
}

impl Drop for WordsDb {
	fn drop(&mut self) {
		if self.new_words.is_empty() {
			return;
		}
		self.words.append(&mut self.new_words);
		self.words.sort_by(|a, b| {
			let r1 = a.len().cmp(&b.len());
			if !r1.is_eq() {
				return r1;
			}
			return a.cmp(&b);
		});

		if self.drop_with_checks().is_err() {
			println!("Unexpected error during saving the word database file");
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_create() {
		let filename = std::path::Path::new("test_create_db");
		if filename.exists() {
			panic!(
				"The file/folder {:?} is exists before the running tests!",
				filename
			);
		}

		{
			let db = WordsDb::new(filename);

			let db = db.unwrap();
			assert!(db.words.is_empty());
		}
		std::fs::remove_file(filename).unwrap();
	}

	#[test]
	fn test_read() {}

	#[test]
	fn test_read_and_write() {}
}
