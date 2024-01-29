use regex::Regex;
use std::{str::FromStr, marker::PhantomData, env::Args};



pub struct ValueArg<T: FromStr> {
	short_name: &'static str,
	long_name: &'static str,
	_owns_t: PhantomData<T>,
}

impl<T: FromStr> ValueArg<T> {
	pub fn new(short_name: &'static str, long_name: &'static str) -> ValueArg<T> {
		Self { short_name, long_name, _owns_t: PhantomData }
	}
	pub fn parse(&self, string: &String) -> Option<T> {
		let pattern = format!(r"(?:\-\-{}|(?:^|\s)\-{})[\s=]([^\s]*)", self.long_name, self.short_name);
		let re = Regex::new(&pattern).unwrap();
		let value = match re.captures(&string) {
			Some(c) => {
				println!("{c:?}");
				c[1].to_owned()
			},
			None => return None,
		};
		T::from_str(&value).ok()
	}
}

pub struct ArgParser {
	argument_str: String,
}
impl ArgParser {
	pub fn new(args: Args) -> Self {
		Self {
			argument_str: args.into_iter().skip_while(|s| !s.starts_with("-")).collect::<Vec<String>>().join(" ")
		}
	}
	pub fn parse_value<T: FromStr>(&self, short_name: &'static str, long_name: &'static str) -> Option<T> {
		let arg = ValueArg::new(short_name, long_name);
		arg.parse(&self.argument_str)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	const ARGS: &'static str = "-b 1.2 --hat true --why lol";
	
	#[test]
	fn test_arg() {
		let arg_parser = ArgParser {
			argument_str: ARGS.to_owned(),
		};

		assert_eq!(arg_parser.parse_value::<f32>("b", "bad"), Some(1.2f32));
		assert_eq!(arg_parser.parse_value::<bool>("h", "hat"), Some(true));
		assert_eq!(arg_parser.parse_value::<i128>("l", "long"), None);
	}
}