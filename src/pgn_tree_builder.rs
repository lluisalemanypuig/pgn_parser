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

use crate::tokenizer;
use crate::game;
use crate::comment;

pub struct PGNTreeBuilder {
	m_tokens: tokenizer::AllTokens,
	m_token_types: tokenizer::AllTokenTypes,
	m_num_tokens: usize,
	
	m_tab: String
}

struct ParseResult {
	pub game: Option<game::Game>,
	pub next: usize
}

impl PGNTreeBuilder {
	pub fn new() -> PGNTreeBuilder {
		PGNTreeBuilder {
			m_tokens: tokenizer::AllTokens::new(),
			m_token_types: tokenizer::AllTokenTypes::new(),
			m_num_tokens: 0,
			m_tab: String::new()
		}
	}
	
	pub fn set_token_list(
		&mut self,
		tokens: tokenizer::AllTokens,
		token_types: tokenizer::AllTokenTypes,
	)
	{
		assert_eq!(self.m_tokens.len(), self.m_token_types.len());
		self.m_tokens = tokens;
		self.m_tokens.reverse();
		self.m_token_types = token_types;
		self.m_num_tokens = self.m_tokens.len();
	}
	
	fn token_index(&self, i: usize) -> usize { self.m_num_tokens - i - 1 }
	fn get_token(&self, i: usize) -> &String { &self.m_tokens[self.token_index(i)] }
	fn remove_token(&mut self, i: usize) -> String {
		self.m_tokens.remove(self.token_index(i))
	}
	
	fn parse_comment_tag(&mut self, mut i: usize) -> (usize, String, String) {
		println!("{}Reading a tag '{}'", self.m_tab, self.get_token(i));
		
		let tag_name = self.remove_token(i);
		i += 1;
		
		let mut text_tag = String::new();
		let mut stop = false;
		while i < self.m_num_tokens && !stop {
			println!("{}{:#?}", self.m_tab, self.get_token(i));
			
			match &self.m_token_types[i] {
				tokenizer::TokenType::CommentDelim { open: false } => {
					panic!("A comment tag was closed with a comment deliminter");
				},
				
				tokenizer::TokenType::TagDelim { open: false } => {
					stop = true;
					self.remove_token(i);
					i += 1;
				},
				
				tokenizer::TokenType::Text => {
					text_tag.push_str( &self.remove_token(i) );
					i += 1;
				},
				
				_ => { }
			}
		}
		
		(i, tag_name, text_tag)
	}
	fn parse_comment(&mut self, mut i: usize)
	-> (comment::Comment, usize)
	{
		println!("{}Parsing comment", self.m_tab);
		
		let mut com = comment::Comment::new();
		let mut text_comment = String::new();
		
		let mut first_text_comment = true;
		let mut stop = false;
		while i < self.m_num_tokens && !stop {
			
			match &self.m_token_types[i] {
				tokenizer::TokenType::CommentDelim { open: false } => {
					println!("{}Exhausted comment", self.m_tab);
					
					stop = true;
					self.remove_token(i);
					i += 1;
				},
				
				tokenizer::TokenType::TagDelim { open: true } => {
					println!("{}Open a tag", self.m_tab);
					
					self.remove_token(i);
					i += 1;
					
					let (next, tag_name, tag_text) = self.parse_comment_tag(i);
					i = next;
					com.add_tag(tag_name, tag_text);
				},
				
				tokenizer::TokenType::Text => {
					println!("{}Read text '{}'", self.m_tab, self.get_token(i));
					
					if first_text_comment {
						first_text_comment = false;
					}
					else {
						text_comment.push_str(" ");
					}
					text_comment.push_str( &self.remove_token(i) );
					i += 1;
				},
				
				_ => { }
			}
		}
		com.set_text(text_comment);
		(com, i)
	}
	
	fn is_variant_or_comment(&self, i: usize) -> bool {
		match &self.m_token_types[i] {
			tokenizer::TokenType::VariantDelim { open: true } => true,
			tokenizer::TokenType::CommentDelim { open: true } => true,
			_ => false
		}
	}
	
	fn build_game_tree_rec(
		&mut self,
		mut i: usize,
		expect_move_id: bool,
		move_number: u32,
		side: tokenizer::Side
	)
	->	ParseResult
	{
		println!("{}A. Token {i}. Depth {move_number} -- {:#?}", self.m_tab, self.m_tokens[self.token_index(i)]);
		
		if i == self.m_num_tokens {
			return ParseResult { game: None, next: self.m_num_tokens };
		}
		
		let mut g = game::Game::new();
		if let tokenizer::TokenType::Result { result: _ } = &self.m_token_types[i] {
			let res = self.remove_token(i);
			g.set_result(res);
			return ParseResult { game: Some(g), next: i };
		}

		if let tokenizer::TokenType::MoveNumber { id, side: sid } = &self.m_token_types[i] {
			println!("{}|-> This is a move number", self.m_tab);
			assert_eq!(move_number, *id);
			assert_eq!(side, *sid);
			self.remove_token(i);
			i += 1;
		}
		else {
			if expect_move_id {
				panic!("I was expecting a move id at move number '{move_number}', side '{:#?}'! Your pgn is probably malformed.", side);
			}
		}
		
		println!("{}B. Token {i}. Depth {move_number} -- {:#?}", self.m_tab, self.m_tokens[self.token_index(i)]);
		
		println!("{}|-> This is an actual move", self.m_tab);
		g.set_move_text(self.remove_token(i), &side, move_number);
		i += 1;
		
		// read a series of variants or comments
		let mut found_variant_comment = false;
		while i < self.m_num_tokens && self.is_variant_or_comment(i) {
			
			match &self.m_token_types[i] {
				tokenizer::TokenType::VariantDelim { open: true } => {
					found_variant_comment = true;

					println!("{}|-> A new variant started", self.m_tab);
					self.remove_token(i);
					
					self.m_tab.push_str("    ");
					let parse = self.build_game_tree_rec(
						i + 1,
						true,
						move_number,
						side.clone()
					);
					self.m_tab.replace_range(0..4, "");
					
					if let ParseResult { game: Some(gg), next } = parse {
						g.add_variation(gg);
						i = next;
						
						if i < self.m_num_tokens {
							println!("{}After parsing the variant...", self.m_tab);
							println!("{}Next {next}. Token {i}. Depth {move_number} -- {:#?}", self.m_tab, self.m_tokens[self.token_index(i)]);
						}
					}
					else {
						panic!("{}Unexpected wrong return", self.m_tab);
					}
				},

				tokenizer::TokenType::CommentDelim { open: true } => {
					found_variant_comment = true;

					println!("{}This is a comment", self.m_tab);
					self.remove_token(i);
					i += 1;
					
					let (comment, next) = self.parse_comment(i);
					g.add_comment(comment);
					i = next;
				}

				_ => {}
			}
		}
		
		if i < self.m_num_tokens {
			println!("{}D. After reading all the variants and comments", self.m_tab);
			println!("{}E. Token {i}. Depth {move_number} -- {:#?}", self.m_tab, self.m_tokens[self.token_index(i)]);
			
			if let tokenizer::TokenType::VariantDelim { open: false } = &self.m_token_types[i] {
				println!("{}F. A variant has been closed. Next {}", self.m_tab, i);
				self.remove_token(i);
				return ParseResult { game: Some(g), next: i + 1 };
			}
			
			println!("{}G. A new move comes", self.m_tab);
			self.m_tab.push_str("    ");
			let next_side = tokenizer::other_side(&side);
			let parse = self.build_game_tree_rec(
				i,
				found_variant_comment,
				move_number + if next_side == tokenizer::Side::White { 1 } else { 0 },
				next_side
			);
			self.m_tab.replace_range(0..4, "");
			
			if let ParseResult { game: Some(gg), next } = parse {
				g.set_next_move(gg);
				i = next;
			}
		}
		
		ParseResult { game: Some(g), next: i }
	}
	
	pub fn build_game_tree(&mut self) -> Option<game::Game> {
		let parse_result = self.build_game_tree_rec(
			0,
			true,
			1,
			tokenizer::Side::White
		);
		
		assert_eq!(self.m_tokens.len(), 0);
		
		parse_result.game
	}
}
