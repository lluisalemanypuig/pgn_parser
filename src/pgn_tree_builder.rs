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
	keep_result: bool,
	data: tokenizer::TokenizedPGN,
	pub tab: String
}

struct ParseResult {
	pub game: Option<game::Game>,
	pub next: usize
}

impl PGNTreeBuilder {
	pub fn new() -> PGNTreeBuilder {
		PGNTreeBuilder {
			keep_result: true,
			data: tokenizer::TokenizedPGN::new(),
			tab: String::new()
		}
	}
	
	pub fn set_keep_result(&mut self, use_res: bool) {
		self.keep_result = use_res;
	}
	
	pub fn set_data(&mut self, data: tokenizer::TokenizedPGN) {
		self.data = data;
	}
	
	fn parse_comment(&self, i: usize)
	-> (comment::Comment, usize)
	{
		(comment::Comment::new(), i + 1)
	}
	
	fn is_variant_or_comment(&self, i: usize) -> bool {
		match &self.data[i].1 {
			tokenizer::TokenType::VariantDelim { open: true } => true,
			tokenizer::TokenType::CommentDelim { open: true } => true,
			_ => false
		}
	}
	
	fn build_game_tree_rec(
		&mut self,
		mut i: usize,
		move_number: u32,
		side: tokenizer::Side
	)
	->	ParseResult
	{
		println!("{}A. Token {i}. Depth {move_number} -- {:#?}", self.tab, self.data[i].0);
		
		if i == self.data.len() {
			return ParseResult { game: None, next: self.data.len() };
		}
		
		if let tokenizer::TokenType::MoveNumber { id, side } = &self.data[i].1 {
			println!("{}|-> This is a move number", self.tab);
			i += 1;
		}
		
		println!("{}B. Token {i}. Depth {move_number} -- {:#?}", self.tab, self.data[i].0);
		let mut g = game::Game::new();
		if let tokenizer::TokenType::Result { result: res } = &self.data[i].1 {
			if self.keep_result {
				g.set_result(self.data[i].0.clone());
				
				return ParseResult { game: Some(g), next: i };
			}
			else {
				return ParseResult { game: None, next: i };
			}
		}
		
		println!("{}|-> This is an actual move", self.tab);
		g.set_move_text(self.data[i].0.clone(), &side, move_number);
		i += 1;
		
		if i < self.data.len() {
			println!("{}C. Token {i}. Depth {move_number} -- {:#?}", self.tab, self.data[i].0);
		}
		
		// read a series of variants or a comment.
		while i < self.data.len() && self.is_variant_or_comment(i) {
			
			match &self.data[i].1 {
				
				tokenizer::TokenType::VariantDelim { open: true } => {
					println!("{}|-> A new variant started", self.tab);
					
					self.tab.push_str("    ");
					let parse = self.build_game_tree_rec(
						i + 1,
						move_number,
						side.clone()
					);
					self.tab.replace_range(0..4, "");
					
					if let ParseResult { game: Some(gg), next: next } = parse {
						
						g.add_variation(gg);
						i = next;
						
						if i < self.data.len() {
							println!("{}After parsing the variant...", self.tab);
							println!("{}Next {next}. Token {i}. Depth {move_number} -- {:#?}", self.tab, self.data[i].0);
						}
					}
					else {
						panic!("{}Unexpected wrong return", self.tab);
					}
				},
				
				tokenizer::TokenType::CommentDelim { open: true } => {
					let (comment, next) = self.parse_comment(i);
					g.set_comment(comment);
					i = next;
				},
				
				_ => { }
			}
		}
		
		if i < self.data.len() {
			println!("{}D. After reading all the variants and comments", self.tab);
			println!("{}E. Token {i}. Depth {move_number} -- {:#?}", self.tab, self.data[i].0);
			
			if let tokenizer::TokenType::VariantDelim { open: false } = &self.data[i].1 {
				println!("{}F. A variant has been closed. Next {}", self.tab, i);
				
				return ParseResult { game: Some(g), next: i + 1 };
			}
			
			println!("{}G. A new move comes", self.tab);
			self.tab.push_str("    ");
			let next_side = tokenizer::other_side(&side);
			let parse = self.build_game_tree_rec(
				i,
				move_number + if next_side == tokenizer::Side::White { 1 } else { 0 },
				next_side
			);
			self.tab.replace_range(0..4, "");
			
			if let ParseResult { game: Some(gg), next: next } = parse {
				g.set_next_move(gg);
				i = next;
			}
		}
		
		ParseResult { game: Some(g), next: i }
	}
	
	pub fn build_game_tree(&mut self) -> Option<game::Game> {
		let parse_result = self.build_game_tree_rec(0, 1, tokenizer::Side::White);
		parse_result.game
	}
}
