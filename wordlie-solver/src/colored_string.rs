use std::fmt::{Display, Formatter};

use colored::Colorize;

#[derive(Eq, PartialEq, Clone)]
pub enum Color {
	Default,
	Gray,
	Yellow,
	Green,
}

pub struct ColoredString {
	string: Vec<char>,
	colors: Vec<Color>,
}

impl ColoredString {
	pub fn new(string: String) -> ColoredString {
		Self {
			string: string.chars().collect(),
			colors: vec![Color::Default; string.len()],
		}
	}

	pub fn set_color(&mut self, idx: usize, color: Color) {
		self.colors[idx] = color;
	}
}

impl Display for ColoredString {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let color_ranges = {
			let mut v = Vec::new();
			let mut cur_col = None;
			let mut idx_start = 0;
			for (idx, col) in self.colors.iter().enumerate() {
				if cur_col != Some(col) {
					if let Some(cur_col) = cur_col {
						v.push((cur_col, &self.string[idx_start..idx]));
					}
					cur_col = Some(col);
					idx_start = idx;
				}
			}
			if let Some(cur_col) = cur_col {
				v.push((cur_col, &self.string[idx_start..]));
			}
			v
		};
		for (c, str_part) in color_ranges {
			let s_tmp: String = str_part.iter().collect();
			let str_part = match c {
				Color::Default => s_tmp.normal(),
				Color::Gray => s_tmp.white(),
				Color::Green => s_tmp.green(),
				Color::Yellow => s_tmp.yellow(),
			};
			write!(f, "{}", str_part)?
		}
		Ok(())
	}
}
