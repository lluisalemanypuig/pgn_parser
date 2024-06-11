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

use crate::game;
use crate::tokenizer;

pub struct GameFormatter {
    m_print_comments: bool,
    m_print_variants: bool,
    m_print_result: bool
}

impl GameFormatter {

	pub fn new() -> GameFormatter {
		GameFormatter {
			m_print_comments: true,
			m_print_variants: true,
			m_print_result: true,
		}
	}

	pub fn set_print_comments(&mut self, v: bool) -> &mut GameFormatter {
		self.m_print_comments = v;
		self
	}
	pub fn set_print_variation(&mut self, v: bool) -> &mut GameFormatter {
		self.m_print_variants = v;
		self
	}
	pub fn set_print_result(&mut self, v: bool) -> &mut GameFormatter {
		self.m_print_result = v;
		self
	}

	fn to_string_rec(&self, g: &game::Game, show_move_number: bool) -> String {
		let mut s = String::new();
		
		if show_move_number {
			if let Some(side) = g.get_side() {
				s.push_str(&g.get_move_number().to_string());
				if side == &tokenizer::Side::White {
					s.push_str(". ");
				}
				else {
					s.push_str("... ");
				}
			}
		}
		
		if !g.is_result() || self.m_print_result {
			s.push_str(g.get_move_text());
		}
		
		let mut show_num_next_move = false;

		if self.m_print_result {
			for c in g.get_comments().iter() {
				show_num_next_move = true;
				s.push_str(" { ");
				
				for tag in c.get_tags().iter() {
					s.push_str("[");
					s.push_str(&tag.0);
					s.push_str(" ");
					s.push_str(&tag.1);
					s.push_str("] ");
				}
				
				s.push_str(c.get_text());
				if c.get_text() != &"".to_string() {
					s.push_str(" ");
				}
				s.push_str("}");
			}
		}
		
		if self.m_print_variants {
			for var in g.get_variations().iter() {
				show_num_next_move = true;
				s.push_str(" (");
				s.push_str(&self.to_string_rec(&var, true));
				s.push_str(")");
			}
		}

		if let Some(res) = g.get_next_move() {
			s.push_str(" ");
			if let Some(next_side) = res.get_side() {
				show_num_next_move = show_num_next_move || next_side == &tokenizer::Side::White;
			}
			s.push_str(&self.to_string_rec(&res, show_num_next_move));
		}
		
		s
	}

	pub fn to_string(&self, g: &game::Game) -> String {
		self.to_string_rec(g, true)
	}
}