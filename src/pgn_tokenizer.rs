/*********************************************************************
 *
 * PGN parser -- A command line utility to process pgn-formatted files.
 *
 * Copyright (C) 2024
 *
 * This file is part of PGN Parser. The full code is available
 * at:
 *      https://github.com/lluisalemanypuig/pgn_parser.git
 *
 * PGN Parser is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * PGN Parser is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with PGN Parser.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Contact:
 *
 *     Lluís Alemany Puig
 *         email: lluis.alemany.puig@gmail.com
 *         https://github.com/lluisalemanypuig
 *         lluisalemanypuig.github.io
 *
 ********************************************************************/

use regex::Regex;

#[derive(Debug,PartialEq)]
enum CharacterType {
	Number,
	Letter,
	Whitespace,
	Quote,
	CurlyBracket(bool),
	SquareBracket(bool),
	Parenthesis(bool),
	Asterisk,
	Other
}

fn classify_char(c: char, in_comment: bool) -> CharacterType {
	
	match c {
		'0'..='9' => if in_comment { CharacterType::Other } else { CharacterType::Number },
		'A'..='Z' => if in_comment { CharacterType::Other } else { CharacterType::Letter },
		'a'..='z' => if in_comment { CharacterType::Other } else { CharacterType::Letter },
		'"' => if in_comment { CharacterType::Other } else { CharacterType::Quote },
		'(' => if in_comment { CharacterType::Other } else { CharacterType::Parenthesis(true) },
		')' => if in_comment { CharacterType::Other } else { CharacterType::Parenthesis(false) },
		'{' => CharacterType::CurlyBracket(true),
		'}' => CharacterType::CurlyBracket(false),
		'[' => CharacterType::SquareBracket(true),
		']' => CharacterType::SquareBracket(false),
		' ' | '　' => CharacterType::Whitespace,
		'*' => CharacterType::Asterisk,
		_ =>  CharacterType::Other
	}
}

#[derive(Debug,Eq,PartialEq,Clone)]
pub enum Side { White, Black }

pub fn other_side(s: &Side) -> Side {
	match s {
		Side::White => Side::Black,
		Side::Black => Side::White
	}
}

#[derive(Debug,PartialEq)]
pub enum ResultType { White, Draw, Black, Unknown }

#[derive(Debug,PartialEq)]
pub enum TokenType {
	VariantDelim { open: bool },
	CommentDelim { open: bool },
	TagDelim { open: bool },
	MoveNumber { id: u16, side: Side },
	Text,
	Result { result: ResultType }
}

pub type AllTokens = Vec<String>;
pub type AllTokenTypes = Vec<TokenType>;

fn is_move_number(s: &String) -> Option<TokenType> {
	let re = Regex::new(r"^(?<move_number>[0-9]+)(?<side>\.+)$").unwrap();
	if let Some(capture) = re.captures(s) {
		Some(TokenType::MoveNumber{
			id: capture["move_number"].parse::<u16>().unwrap(),
			side: if capture["side"] == ".".to_string() { Side::White } else { Side::Black }
		})
	}
	else {
		None
	}
}

fn is_result_tag(s: &String) -> Option<TokenType> {
	if s == "*" {
		return Some(TokenType::Result { result: ResultType::Unknown })
	}
	
	if !s.contains("-") {
		return None;
	}
	
	if s == "1-0" {
		return Some(TokenType::Result { result: ResultType::White })
	}
	if s == "1/2-1/2" {
		return Some(TokenType::Result { result: ResultType::Draw })
	}
	if s == "0-1" {
		return Some(TokenType::Result { result: ResultType::Black })
	}

	return None
}

fn add_token(s: String, tokens: &mut AllTokens, token_types: &mut AllTokenTypes) {
	if let Some(move_number) = is_move_number(&s) {
		tokens.push(s);
		token_types.push(move_number);
		return;
	}
	if let Some(result) = is_result_tag(&s) {
		tokens.push(s);
		token_types.push(result);
		return;
	}
	
	if s != "".to_string() {
		tokens.push(s);
		token_types.push(TokenType::Text);
	}
}

pub fn tokenize(s: String) -> (AllTokens, AllTokenTypes) {
	let mut tokens: AllTokens = Vec::new();
	let mut token_types: AllTokenTypes = Vec::new();

	let mut next_str: String = String::new();

	let mut in_comment = false;
	let mut open_quote = false;
	for c in s.chars() {
		
		match classify_char(c, in_comment) {
			CharacterType::Number |
			CharacterType::Letter |
			CharacterType::Asterisk |
			CharacterType::Other => next_str.push(c),

			CharacterType::Quote => {
				next_str.push(c);
				if open_quote {
					open_quote = false;
					add_token(next_str, &mut tokens, &mut token_types);
					next_str = String::new();
				}
				else {
					open_quote = true;
				}
			},

			CharacterType::Whitespace => {
				if !open_quote {
					if next_str != "".to_string() {
						add_token(next_str, &mut tokens, &mut token_types);
						next_str = String::new();
					}
				}
			},
			CharacterType::Parenthesis(o) => {
				if !open_quote {
					if next_str != "".to_string() {
						add_token(next_str, &mut tokens, &mut token_types);
					}
					tokens.push(c.to_string());
					token_types.push(TokenType::VariantDelim{open: o});
					next_str = String::new();
				}
			},
			CharacterType::CurlyBracket(o) => {
				in_comment = o;

				if !open_quote {
					if next_str != "".to_string() {
						add_token(next_str, &mut tokens, &mut token_types);
					}
					tokens.push(c.to_string());
					token_types.push(TokenType::CommentDelim{open: o});
					next_str = String::new();
				}
			},
			CharacterType::SquareBracket(o) => {
				if !open_quote {
					if next_str != "".to_string() {
						add_token(next_str, &mut tokens, &mut token_types);
					}
					tokens.push(c.to_string());
					token_types.push(TokenType::TagDelim{open: o});
					next_str = String::new();
				}
			}
		}
	}
	add_token(next_str, &mut tokens, &mut token_types);

	assert_eq!(tokens.len(), token_types.len());
	(tokens, token_types)
}

