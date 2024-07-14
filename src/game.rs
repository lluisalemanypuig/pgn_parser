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

use crate::comment;
use crate::pgn_tokenizer;

#[derive(Debug)]
pub struct Game {
	m_game_move: String,
	m_is_result: bool,
	m_move_number: u32,
	m_side: Option<pgn_tokenizer::Side>,
	m_comments: Vec<comment::Comment>,
	
	m_main_line_next: Option<Box<Game>>,
	m_variations: Vec<Game>,
}

impl Game {
	pub fn new() -> Game {
		Game {
			m_game_move: "".to_string(),
			m_is_result: false,
			m_move_number: 0,
			m_side: None,
			m_comments: Vec::new(),
			
			m_main_line_next: None,
			m_variations: Vec::new(),
		}
	}
	
	/* MODIFIERS */

	pub fn set_move_text
	(&mut self, text: String, s: &pgn_tokenizer::Side, num: u32)
	{
		self.m_game_move = text;
		self.m_side = Some(s.clone());
		self.m_move_number = num;
	}

	pub fn set_result(&mut self, text: String) {
		self.m_game_move = text;
		self.m_is_result = true;
		self.m_side = None;
		self.m_move_number = 0;
	}

	pub fn set_next_move(&mut self, game: Game) {
		self.m_main_line_next = Some(Box::new(game));
	}

	pub fn add_variation(&mut self, variation: Game) {
		self.m_variations.push(variation);
	}

	pub fn add_comment(&mut self, comment: comment::Comment) {
		self.m_comments.push(comment);
	}
	
	/* GETTERS */
	
	pub fn get_side(&self) -> &Option<pgn_tokenizer::Side> { &self.m_side }
	pub fn get_move_text(&self) -> &String { &self.m_game_move }
	pub fn get_move_number(&self) -> &u32 { &self.m_move_number }
	pub fn get_next_move(&self) -> &Option<Box<Game>> { &self.m_main_line_next }
	//pub fn is_move_empty(&self) -> bool { !self.is_result() && self.m_game_move == "".to_string() }
	pub fn is_result(&self) -> bool { self.m_is_result }
	pub fn get_variations(&self) -> &Vec<Game> { &self.m_variations }
	pub fn get_comments(&self) -> &Vec<comment::Comment> { &self.m_comments }
	
}

impl Drop for Game {
	fn drop(&mut self) {
		let mut next_game = self.m_main_line_next.take();
		while let Some(mut game) = next_game {
			next_game = game.m_main_line_next.take();
		}

		// No need to drop m_variations since this is handled automatically
		// by Rust. The deallocation of each element in m_variations is handled
		// by this function.
	}
}
