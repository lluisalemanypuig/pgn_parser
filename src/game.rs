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
use crate::tokenizer;

#[derive(Debug)]
pub struct Game {
	m_game_move: String,
	m_is_result: bool,
	m_move_number: u32,
	m_side: Option<tokenizer::Side>,
	
	m_main_line_next: Option<Box<Game>>,
	m_variations: Vec<Game>,
	m_comment: Option<comment::Comment>,
}

impl Game {
	pub fn new() -> Game {
		Game {
			m_game_move: "".to_string(),
			m_is_result: false,
			m_move_number: 0,
			m_side: None,
			
			m_main_line_next: None,
			m_variations: Vec::new(),
			m_comment: None,
		}
	}
	
	pub fn set_move_text(
		&mut self,
		text: String,
		s: &tokenizer::Side,
		num: u32
	)
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

	pub fn set_comment(&mut self, comment: comment::Comment) {
		self.m_comment = Some(comment);
	}
	
	/* GETTERS */
	
	pub fn get_move_text(&self) -> &String {
		&self.m_game_move
	}
	pub fn get_move(&self) -> &Option<Box<Game>> {
		&self.m_main_line_next
	}
	pub fn empty_move(&self) -> bool {
		self.m_game_move == "".to_string()
	}
	pub fn get_m_variations(&self) -> &Vec<Game> {
		&self.m_variations
	}
	
	pub fn to_string_rec(
		&self,
		show_move_number: bool
	)
	-> String
	{
		let mut s = String::new();
		
		if self.m_side == Some(tokenizer::Side::White) {
			s.push_str( &self.m_move_number.to_string() );
			s.push_str(". ");
		}
		else if show_move_number {
			s.push_str( &self.m_move_number.to_string() );
			s.push_str("... ");
		}
		
		s.push_str( &self.m_game_move.clone() );
		
		if self.m_comment.is_some() {
			s.push_str(" { ");
			let com = self.m_comment.as_ref().unwrap();
			
			for tag in com.get_tags().iter() {
				s.push_str("[");
				s.push_str(&tag.0);
				s.push_str(" ");
				s.push_str(&tag.1);
				s.push_str("] ");
			}
			
			s.push_str(com.get_text());
			if com.get_text() != &"".to_string() {
				s.push_str(" ");
			}
			s.push_str("}");
		}
		
		let exist_variations = self.m_variations.len() > 0;
		if exist_variations {
			for var in self.m_variations.iter() {
				s.push_str(" (");
				s.push_str(&var.to_string_rec( true ));
				s.push_str(")");
			}
		}
		if let Some(res) = &self.m_main_line_next {
			s.push_str(" ");
			s.push_str(&res.to_string_rec(exist_variations));
		}
		
		s
	}
	
	pub fn to_string(&self) -> String {
		self.to_string_rec( false )
	}
}

pub fn print_game(g: &Game) {
	print!("'{}'", g.to_string());
}
