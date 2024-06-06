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

#[derive(Debug)]
pub enum Side { White, Black }

#[derive(Debug)]
pub enum ResultType { White, Draw, Black }

#[derive(Debug)]
pub enum TokenType {
	VariantDelim { open: bool },
	CommentDelim { open: bool },
	TagDelim { open: bool },
	MoveNumber { id: u32, side: Side },
	Text,
	Result { result: ResultType }
}

pub type TokenizedPGN = Vec<(String, TokenType)>;

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

pub fn tokenize(s: String) -> TokenizedPGN {
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

