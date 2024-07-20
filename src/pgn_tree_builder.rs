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

use crate::pgn_tokenizer;
use crate::game;
use crate::comment;

pub struct PGNTreeBuilder {
	m_tokens: pgn_tokenizer::AllTokens,
	m_token_types: pgn_tokenizer::AllTokenTypes,
	m_num_tokens: usize
}

struct ParseResult {
	pub game: Option<game::GameTree>,
	pub next: usize
}

impl PGNTreeBuilder {
	pub fn new() -> PGNTreeBuilder {
		PGNTreeBuilder {
			m_tokens: pgn_tokenizer::AllTokens::new(),
			m_token_types: pgn_tokenizer::AllTokenTypes::new(),
			m_num_tokens: 0
		}
	}
	
	pub fn set_token_list(
		&mut self,
		tokens: pgn_tokenizer::AllTokens,
		token_types: pgn_tokenizer::AllTokenTypes,
	)
	{
		assert_eq!(self.m_tokens.len(), self.m_token_types.len());
		self.m_tokens = tokens;
		self.m_tokens.reverse();
		self.m_token_types = token_types;
		self.m_num_tokens = self.m_tokens.len();
	}
	
	fn token_index(&self, i: usize) -> usize { self.m_num_tokens - i - 1 }
	fn remove_token(&mut self, i: usize) -> String {
		self.m_tokens.remove(self.token_index(i))
	}
	
	fn parse_comment_tag(&mut self, mut i: usize) -> (usize, String, String) {
		let tag_name = self.remove_token(i);
		i += 1;
		
		let mut text_tag = String::new();
		let mut stop = false;
		while i < self.m_num_tokens && !stop {
			
			match &self.m_token_types[i] {
				pgn_tokenizer::TokenType::CommentDelim { open: false } => {
					panic!("A comment tag was closed with a comment deliminter");
				},
				
				pgn_tokenizer::TokenType::TagDelim { open: false } => {
					stop = true;
					self.remove_token(i);
					i += 1;
				},
				
				pgn_tokenizer::TokenType::Text => {
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
		let mut com = comment::Comment::new();
		let mut text_comment = String::new();
		
		let mut first_text_comment = true;
		let mut stop = false;
		while i < self.m_num_tokens && !stop {
			
			match &self.m_token_types[i] {
				pgn_tokenizer::TokenType::CommentDelim { open: false } => {
					stop = true;
					self.remove_token(i);
					i += 1;
				},
				
				pgn_tokenizer::TokenType::TagDelim { open: true } => {
					self.remove_token(i);
					i += 1;
					
					let (next, tag_name, tag_text) = self.parse_comment_tag(i);
					i = next;
					com.add_tag(tag_name, tag_text);
				},
				
				pgn_tokenizer::TokenType::Text => {
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
			pgn_tokenizer::TokenType::VariantDelim { open: true } => true,
			pgn_tokenizer::TokenType::CommentDelim { open: true } => true,
			_ => false
		}
	}
	
	fn build_game_tree_rec(
		&mut self,
		mut i: usize,
		expect_move_id: bool,
		move_number: u32,
		side: pgn_tokenizer::Side
	)
	->	ParseResult
	{
		if i == self.m_num_tokens {
			return ParseResult { game: None, next: self.m_num_tokens };
		}
		
		let mut g = game::GameTree::new();
		if let pgn_tokenizer::TokenType::Result { result: _ } = &self.m_token_types[i] {
			let res = self.remove_token(i);
			g.set_result(res);
			return ParseResult { game: Some(g), next: i };
		}

		if let pgn_tokenizer::TokenType::MoveNumber { id, side: sid } = &self.m_token_types[i] {
			assert_eq!(move_number, *id);
			assert_eq!(side, *sid);
			self.remove_token(i);
			i += 1;
		}
		else if expect_move_id {
			panic!("I was expecting a move id at move number '{move_number}', side '{:#?}'! Your pgn is probably malformed.", side);
		}
		
		g.set_move_text(self.remove_token(i), &side, move_number);
		i += 1;
		
		// read a series of variants or comments
		let mut found_variant_comment = false;
		while i < self.m_num_tokens && self.is_variant_or_comment(i) {
			
			match &self.m_token_types[i] {
				pgn_tokenizer::TokenType::VariantDelim { open: true } => {
					found_variant_comment = true;

					self.remove_token(i);
					
					let parse = self.build_game_tree_rec(
						i + 1,
						true,
						move_number,
						side.clone()
					);
					
					if let ParseResult { game: Some(gg), next } = parse {
						g.add_variation(gg);
						i = next;
					}
					else {
						panic!("Unexpected wrong return");
					}
				},

				pgn_tokenizer::TokenType::CommentDelim { open: true } => {
					found_variant_comment = true;

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
			
			if let pgn_tokenizer::TokenType::VariantDelim { open: false } = &self.m_token_types[i] {
				self.remove_token(i);
				return ParseResult { game: Some(g), next: i + 1 };
			}
			
			let next_side = pgn_tokenizer::other_side(&side);
			let parse = self.build_game_tree_rec(
				i,
				found_variant_comment,
				move_number + if next_side == pgn_tokenizer::Side::White { 1 } else { 0 },
				next_side
			);
			
			if let ParseResult { game: Some(gg), next } = parse {
				g.set_next_move(gg);
				i = next;
			}
		}
		
		ParseResult { game: Some(g), next: i }
	}
	
	pub fn build_game_tree(&mut self) -> Option<game::GameTree> {
		let parse_result = self.build_game_tree_rec(
			0,
			true,
			1,
			pgn_tokenizer::Side::White
		);
		
		assert_eq!(self.m_tokens.len(), 0);
		
		parse_result.game
	}
}
