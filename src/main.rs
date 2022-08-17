use std::env;

fn get_word_len() -> Option<i32> {
	let mut world_len_arg = false;
	for arg in env::args() {
		if arg == "-w" {
			world_len_arg = true;
		} else {
			if world_len_arg {
				let tmp_res = arg.trim().parse::<i32>();
				if let Ok(res) = tmp_res {
					return Some(res);
				}
				world_len_arg = false;
			}
		}
	}
	return None;
}

fn main() {
	let word_len = get_word_len();
	if word_len.is_none() {
		println!("The length of words is not set. Call the program with -w <len> arguments");
		return;
	}
	let word_len = word_len.unwrap();
	println!("Word len is {}", word_len);
}
