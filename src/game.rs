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

#[derive(Debug,Eq,PartialEq)]
pub struct GameTree {
	m_game_move: String,
	m_is_result: bool,
	m_move_number: u32,
	m_side: Option<pgn_tokenizer::Side>,
	m_comments: Vec<comment::Comment>,
	
	m_next: Option<Box<GameTree>>,
	m_variations: Vec<GameTree>,
}

impl GameTree {
	pub fn new() -> GameTree {
		GameTree {
			m_game_move: "".to_string(),
			m_is_result: false,
			m_move_number: 0,
			m_side: None,
			m_comments: Vec::new(),
			
			m_next: None,
			m_variations: Vec::new(),
		}
	}
	pub fn new_data(
		game_move: String,
		is_result: bool,
		move_number: u32,
		side: Option<pgn_tokenizer::Side>,
		comments: Vec<comment::Comment>,
		main_line_next: Option<Box<GameTree>>,
		variations: Vec<GameTree>
	)
	-> GameTree
	{
		GameTree {
			m_game_move: game_move,
			m_is_result: is_result,
			m_move_number: move_number,
			m_side: side,
			m_comments: comments,
			
			m_next: main_line_next,
			m_variations: variations,
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

	pub fn set_next_move(&mut self, game: GameTree) {
		self.m_next = Some(Box::new(game));
	}

	pub fn add_variation(&mut self, variation: GameTree) {
		self.m_variations.push(variation);
	}

	pub fn add_comment(&mut self, comment: comment::Comment) {
		self.m_comments.push(comment);
	}
	
	/* GETTERS */
	
	pub fn get_side(&self) -> &Option<pgn_tokenizer::Side> { &self.m_side }
	pub fn get_move_text(&self) -> &String { &self.m_game_move }
	pub fn get_move_number(&self) -> &u32 { &self.m_move_number }
	pub fn get_next_move(&self) -> &Option<Box<GameTree>> { &self.m_next }
	pub fn get_next_move_mut(&mut self) -> &mut Option<Box<GameTree>> { &mut self.m_next }
	//pub fn is_move_empty(&self) -> bool { !self.is_result() && self.m_game_move == "".to_string() }
	pub fn is_result(&self) -> bool { self.m_is_result }
	pub fn get_variations(&self) -> &Vec<GameTree> { &self.m_variations }
	pub fn get_comments(&self) -> &Vec<comment::Comment> { &self.m_comments }
	
}

impl Drop for GameTree {
	fn drop(&mut self) {
		let mut next_game = self.m_next.take();
		while let Some(mut game) = next_game {
			next_game = game.m_next.take();
		}

		// No need to drop m_variations since this is handled automatically
		// by Rust. The deallocation of each element in m_variations is handled
		// by this function.
	}
}

pub enum TagType {
	Event,
	Site,
	Date,
	Round,
	White,
	Black,
	Result,
	WhiteElo,
	WhiteTeam,
	WhiteTitle,
	BlackElo,
	BlackTeam,
	BlackTitle,
	TimeControl,
	Termination,
	Board,
	Annotator,
	Variant,
	ECO,
	Opening,
	Other(String)
}

pub fn classify(s: String) -> TagType {
	if s == "Event".to_string() { return TagType::Event; }
	if s == "Site".to_string() { return TagType::Site; }
	if s == "Date".to_string() { return TagType::Date; }
	if s == "Round".to_string() { return TagType::Round; }
	if s == "White".to_string() { return TagType::White; }
	if s == "Black".to_string() { return TagType::Black; }
	if s == "Result".to_string() { return TagType::Result; }
	if s == "WhiteElo".to_string() { return TagType::WhiteElo; }
	if s == "WhiteTeam".to_string() { return TagType::WhiteTeam; }
	if s == "WhiteTitle".to_string() { return TagType::WhiteTitle; }
	if s == "BlackElo".to_string() { return TagType::BlackElo; }
	if s == "BlackTeam".to_string() { return TagType::BlackTeam; }
	if s == "BlackTitle".to_string() { return TagType::BlackTitle; }
	if s == "TimeControl".to_string() { return TagType::TimeControl; }
	if s == "Termination".to_string() { return TagType::Termination; }
	if s == "Board".to_string() { return TagType::Board; }
	if s == "Annotator".to_string() { return TagType::Annotator; }
	if s == "Variant".to_string() { return TagType::Variant; }
	if s == "ECO".to_string() { return TagType::ECO; }
	if s == "Opening".to_string() { return TagType::Opening; }
	return TagType::Other(s);
}

pub struct Game {
	m_tree: GameTree,
	m_tags: Vec<(TagType,String)>
}

impl Game {
	pub fn new() -> Game {
		Game {
			m_tree: GameTree::new(),
			m_tags: Vec::new()
		}
	}

	pub fn set_tree(&mut self, tree: GameTree) {
		self.m_tree = tree;
	}

	pub fn add_game_tag(&mut self, tag: (TagType, String)) {
		self.m_tags.push(tag);
	}

	pub fn get_tree(&self) -> &GameTree {
		&self.m_tree
	}
}
