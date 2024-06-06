use std::env;
use std::io::{BufRead};

use regex::Regex;

mod game;

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
enum ResultType { White, Draw, Black }

#[derive(Debug)]
enum TokenType {
	VariantDelim { open: bool },
	CommentDelim { open: bool },
	TagDelim { open: bool },
	MoveNumber { id: u32, side: Side },
	Text,
	Result { result: ResultType }
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

type TokenizedPGN = Vec<(String, TokenType)>;

fn is_move_number(str: &String) -> Option<TokenType> {
	let re = Regex::new(r"^(?<move_number>[0-9]+)(?<side>\.+)$").unwrap();
	if let Some(caps) = re.captures(str) {
		Some(TokenType::MoveNumber{
			id: caps["move_number"].parse::<u32>().unwrap(),
			side: if caps["side"] == ".".to_string() { Side::White } else { Side::Black }
		})
	}
	else {
		None
	}
}

fn is_result_tag(str: &String) -> Option<TokenType> {
	if str.contains("-") {
		if str == "1-0" {
			return Some(TokenType::Result { result: ResultType::White });
		}
		else if str == "1/2-1/2" {
			return Some(TokenType::Result { result: ResultType::Draw });
		}
		else if str == "0-1" {
			return Some(TokenType::Result { result: ResultType::Black });
		}
		else {
			return None;
		}
	}
	None
}

fn add_token(str: String, res: &mut TokenizedPGN) {
	println!("Adding string '{str}'");

	if let Some(move_number) = is_move_number(&str) {
		res.push( (str, move_number) );
		return;
	}
	if let Some(result) = is_result_tag(&str) {
		res.push( (str, result) );
		return;
	}
	
	res.push( (str, TokenType::Text) );
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
				res.push((c.to_string(), TokenType::VariantDelim{open: o} ));
				next_str = "".to_string();
			},
			CharacterType::CurlyBracket(o) => {
				if next_str != "".to_string() {
					add_token(next_str, &mut res);
				}
				res.push((c.to_string(), TokenType::CommentDelim{open: o} ));
				next_str = "".to_string();
			},
			CharacterType::SquareBracket(o) => {
				if next_str != "".to_string() {
					add_token(next_str, &mut res);
				}
				res.push((c.to_string(), TokenType::TagDelim{open: o} ));
				next_str = "".to_string();
			}
		}
	}
	add_token(next_str, &mut res);

	res
}



fn parse_comment(data: &TokenizedPGN, i: usize) -> (game::Comment, usize) {
	(game::Comment::new(), i + 1)
}

fn build_pgn_tree(data: &TokenizedPGN, mut i: usize) -> (
	Option<game::Game>,
	bool,
	usize
)
{
	let mut g = game::Game::new();
	let total_length = data.len();
	
	while i < total_length {
		println!("{i} -- {:#?}", data[i]);
		
		match &data[i].1 {
			TokenType::MoveNumber { id, side } => {
				i += 1;
			},
			
			TokenType::Text => {
				g.set_move_text(data[i].0.clone());
				
				let (result, finished_variation, next) = build_pgn_tree(&data, i + 1);
				if let Some(rest) = result {
					g.set_next_move(rest);
				}
				i = next;
				
				if finished_variation {
					return (Some(g), true, i);
				}
			},
			
			TokenType::CommentDelim { open: o } => {
				if *o == false {
					panic!("Unexpected closed comment delimiter at token {i}");
				}
				
				let (com, next) = parse_comment(&data, i + 1);
				g.set_comment(com);
				i = next;
			},
			
			TokenType::VariantDelim { open: true } => {
				println!("Started a variation at {i}");
				
				let (res, _, next) = build_pgn_tree(&data, i + 1);
				
				println!("Variation...");
				println!("{:#?}", res);
				
				if let Some(var) = res {
					println!("Add the variation to the game...");
					g.add_variation(var);
				}
				i = next;
			},
			
			TokenType::VariantDelim { open: false } => {
				println!("Finished a variation at {i}");
				return (None, true, i + 1);
			},
			
			_ => {
				i += 1;
			}
		}
	}
	
	(None, true, i + 1)
}

fn analyze_file(p: String) {
	println!("Opening file: {p}");
	let mut entire_file_str = String::new();

	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		entire_file_str.push_str( line.unwrap().trim() );
	}

	let res = tokenize(entire_file_str);
	for str in res.iter() {
		println!("{:?} -- {:?}", str.0, str.1);
	}

	if let (Some(game), _, _) = build_pgn_tree(&res, 0) {
		println!("{:#?}", game);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	analyze_file(args[1].clone());
}
