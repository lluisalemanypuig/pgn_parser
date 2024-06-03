use std::io::{BufRead, Write, Result};

use regex::Regex;

#[derive(Debug,PartialEq)]
enum CharacterType {
	Number,
	Letter,
	Whitespace,
	CurlyBracket(bool),
	SquareBracket(bool),
	Parenthesis(bool),
	Other
}

#[derive(Debug)]
enum Side { White, Black }

#[derive(Debug)]
enum StringType {
	VariantDelim { open: bool },
	CommentDelim { open: bool },
	TagDelim { open: bool },
	MoveNumber { id: u32, side: Side },
	MoveText,
	Other
}

fn classify_char(c: char) -> CharacterType {
	match c {
		'0'..='9' => CharacterType::Number,
		'A'..='Z' => CharacterType::Letter,
		'a'..='z' => CharacterType::Letter,
		'(' => CharacterType::Parenthesis(true),
		')' => CharacterType::Parenthesis(false),
		'{' => CharacterType::CurlyBracket(true),
		'}' => CharacterType::CurlyBracket(false),
		'[' => CharacterType::SquareBracket(true),
		']' => CharacterType::SquareBracket(false),
		' ' | '　' => CharacterType::Whitespace,
		_ =>  CharacterType::Other
	}
}

type TokenizedPGN = Vec<(String, StringType)>;

fn is_move_number(str: &String) -> Option<StringType> {
	let re = Regex::new(r"^(?<move_number>[0-9]+)(?<side>\.+)$").unwrap();
	if let Some(caps) = re.captures(str) {
		Some(StringType::MoveNumber{
			id: caps["move_number"].parse::<u32>().unwrap(),
			side: if caps["side"] == ".".to_string() { Side::White } else { Side::Black }
		})
	}
	else {
		None
	}
}

fn add_token(str: String, res: &mut TokenizedPGN) {
	println!("Adding string '{str}'");

	if let Some(move_number) = is_move_number(&str) {
		res.push( (str, move_number) );
		return;
	}
	
	res.push( (str, StringType::Other) );
}

fn tokenize(s: String) -> TokenizedPGN {
	let mut res: TokenizedPGN = Vec::new();

	let mut next_str: String = String::new();
	for c in s.chars() {
		
		match classify_char(c) {
			CharacterType::Number | CharacterType::Letter | CharacterType::Other => next_str.push(c),
			CharacterType::Whitespace => {
				if next_str != "".to_string() {
					add_token(next_str, &mut res);
					next_str = "".to_string();
				}
			},
			CharacterType::Parenthesis(o) => {
				if next_str != "".to_string() {
					add_token(next_str, &mut res);
				}
				res.push((c.to_string(), StringType::VariantDelim{open: o} ));
				next_str = "".to_string();
			},
			CharacterType::CurlyBracket(o) => {
				if next_str != "".to_string() {
					add_token(next_str, &mut res);
				}
				res.push((c.to_string(), StringType::CommentDelim{open: o} ));
				next_str = "".to_string();
			},
			CharacterType::SquareBracket(o) => {
				if next_str != "".to_string() {
					add_token(next_str, &mut res);
				}
				res.push((c.to_string(), StringType::TagDelim{open: o} ));
				next_str = "".to_string();
			}
		}
	}
	res.push((next_str, StringType::Other));

	res
}

fn main() {
	let p = "sample_games/fg_01.pgn";
	let mut entire_file_str = String::new();

	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		entire_file_str.push_str( line.unwrap().trim() );
	}

	let res = tokenize(entire_file_str);
	for str in res {
		println!("'{str:?}'");
	}
}
