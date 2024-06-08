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

#[derive(Debug,PartialEq,Clone)]
pub enum Side { White, Black }

pub fn other_side(s: &Side) -> Side {
	match s {
		Side::White => Side::Black,
		Side::Black => Side::White
	}
}

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

fn add_token(s: String, res: &mut TokenizedPGN) {
	if let Some(move_number) = is_move_number(&s) {
		res.push( (s, move_number) );
		return;
	}
	if let Some(result) = is_result_tag(&s) {
		res.push( (s, result) );
		return;
	}
	
	if s != "".to_string() {
		res.push( (s, TokenType::Text) );
	}
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

