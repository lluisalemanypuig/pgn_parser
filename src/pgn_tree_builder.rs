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
		self.m_token_types = token_types;
		self.m_num_tokens = self.m_tokens.len();
	}
	
	fn retrieve_token(&mut self, i: usize) -> String {
		std::mem::replace(&mut self.m_tokens[i], String::new())
	}
	
	fn parse_comment_tag(&mut self, mut i: usize) -> (usize, String, String) {
		let tag_name = self.retrieve_token(i);
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
					i += 1;
				},
				
				pgn_tokenizer::TokenType::Text => {
					text_tag.push_str( &self.retrieve_token(i) );
					i += 1;
				},
				
				_ => { }
			}
		}
		
		(i, tag_name, text_tag)
	}
	
	fn parse_comment(&mut self, mut i: usize) -> (comment::Comment, usize) {
		let mut com = comment::Comment::new();
		let mut text_comment = String::new();
		
		let mut first_text_comment = true;
		let mut stop = false;
		while i < self.m_num_tokens && !stop {
			
			match &self.m_token_types[i] {
				pgn_tokenizer::TokenType::CommentDelim { open: false } => {
					stop = true;
					i += 1;
				},
				
				pgn_tokenizer::TokenType::TagDelim { open: true } => {
					i += 1;
					
					let (next, tag_name, tag_text) = self.parse_comment_tag(i);
					i = next;
					com.add_tag(
						comment::classify_tag(tag_name),
						tag_text
					);
				},
				
				pgn_tokenizer::TokenType::Text => {
					if first_text_comment {
						first_text_comment = false;
					}
					else {
						text_comment.push_str(" ");
					}
					text_comment.push_str( &self.retrieve_token(i) );
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
		move_number: u16,
		expect_move_id: bool,
		side: pgn_tokenizer::Side
	)
	->	ParseResult
	{
		if i == self.m_num_tokens {
			return ParseResult { game: None, next: self.m_num_tokens };
		}
		
		let mut g = game::GameTree::new();
		if let pgn_tokenizer::TokenType::Result { result: _ } = &self.m_token_types[i] {
			let res = self.retrieve_token(i);
			g.set_result(res);
			return ParseResult { game: Some(g), next: i };
		}

		if let pgn_tokenizer::TokenType::MoveNumber { id, side: sid } = &self.m_token_types[i] {
			assert_eq!(move_number, *id);
			assert_eq!(side, *sid);
			i += 1;
		}
		else if expect_move_id {
			panic!(
				"I was expecting a move id at move number '{move_number}', side '{:#?}'. \
				Instead, I found token {:#?} of type '{:#?}'. \
				Your pgn is probably malformed.",
				side,
				self.m_tokens[i],
				self.m_token_types[i]
			);
		}
		
		g.set_move_text(self.retrieve_token(i), &side, move_number);
		i += 1;
		
		// read a series of variants or comments
		let mut found_variant_comment = false;
		while i < self.m_num_tokens && self.is_variant_or_comment(i) {
			
			match &self.m_token_types[i] {
				pgn_tokenizer::TokenType::VariantDelim { open: true } => {
					found_variant_comment = true;

					let parse = self.build_game_tree_rec(
						i + 1,
						move_number,
						true,
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
				return ParseResult { game: Some(g), next: i + 1 };
			}
			
			let next_side = pgn_tokenizer::other_side(&side);
			let next_move = move_number + if next_side == pgn_tokenizer::Side::White { 1 } else { 0 };
			let parse = self.build_game_tree_rec(
				i,
				next_move,
				found_variant_comment,
				next_side
			);
			
			if let ParseResult { game: Some(gg), next } = parse {
				g.set_next_move(gg);
				i = next;
			}
		}
		
		ParseResult { game: Some(g), next: i }
	}
	
	pub fn build_game_tree(&mut self, i: usize) -> Option<game::GameTree> {

		let parse_result = self.build_game_tree_rec(
			i,
			1,
			true,
			pgn_tokenizer::Side::White
		);
		
		parse_result.game
	}

	pub fn build_game(&mut self) -> Option<game::Game> {
		let mut g = game::Game::new();

		let mut i = 0;
		while let pgn_tokenizer::TokenType::TagDelim { open: true } = &self.m_token_types[i] {
			let tag_type = game::classify(self.retrieve_token(i + 1));
			g.add_game_tag((tag_type, self.retrieve_token(i + 2)));

			i += 4;
		}

		let parse_result = self.build_game_tree(i);

		g.set_tree(parse_result.unwrap());

		Some(g)
	}
}
