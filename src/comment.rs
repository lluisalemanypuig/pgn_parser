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

 #[derive(Debug,Eq,PartialEq)]
pub enum TagType {
	Clock,
	Eval,
	Other(String)
}

pub fn classify_tag(s: String) -> TagType {
	if s == "%clk".to_string() { return TagType::Clock; }
	if s == "%eval".to_string() { return TagType::Eval; }
	return TagType::Other(s);
}

pub fn tag_to_string(t: &TagType) -> String {
	match &t {
		TagType::Clock => "%clk".to_string(),
		TagType::Eval => "%eval".to_string(),
		TagType::Other(s) => s.clone()
	}
}

#[derive(Debug,Eq,PartialEq)]
pub struct Comment {
	m_text: String,
	m_tags: Vec<(TagType, String)>
}

impl Comment {
	pub fn new() -> Comment {
		Comment {
			m_text: String::new(),
			m_tags: Vec::new()
		}
	}
	pub fn new_data(text: String, tags: Vec<(TagType,String)>) -> Comment {
		Comment {
			m_text: text,
			m_tags: tags
		}
	}
	
	/* GETTERS */
	
	pub fn get_text(&self) -> &String { &self.m_text }
	pub fn get_tags(&self) -> &Vec<(TagType, String)> { &self.m_tags }
	
	/* MODIFIERS */
	
	pub fn set_text(&mut self, text: String) {
		self.m_text = text;
	}
	pub fn add_tag(&mut self, tag_name: TagType, tag_text: String) {
		self.m_tags.push((tag_name, tag_text));
	}
}
