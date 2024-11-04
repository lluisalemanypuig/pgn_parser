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
 * Contact:
 *
 *     Lluís Alemany Puig
 *         email: lluis.alemany.puig@gmail.com
 *         https://github.com/lluisalemanypuig
 *         lluisalemanypuig.github.io
 *
 *
 ********************************************************************/

use std::env;
use std::io::BufRead;

mod comment;
mod game;
mod pgn_formatter;
mod pgn_tokenizer;
mod pgn_tree_builder;

fn analyze_file(p: String) -> game::Game {
	let mut entire_file_str = String::new();
	
	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		entire_file_str.push_str( line.unwrap().trim() );
	}
	
	let (all_tokens, all_token_types) =
		pgn_tokenizer::tokenize(entire_file_str);

	let mut builder = pgn_tree_builder::PGNTreeBuilder::new();
	builder.set_token_list(all_tokens, all_token_types);

	/*
	if let Some(game) = builder.build_game() {
		let res = pgn_formatter::PgnFormatter::new()
			.set_print_comments(true)
			.set_print_variation(true)
			.set_print_result(true)
			.to_string(game.get_tree());

		println!("{res}");
		println!("");
	}
	*/

	builder.build_game().unwrap()
}

pub fn read_input_string() -> String {
	let mut s = String::new();
	let stdin = std::io::stdin();
	stdin.read_line(&mut s).expect("I was expecting standard input");
	s.trim().to_string()
}

pub fn read_string_or_empty() -> Option<String> {
	let str = read_input_string();
	if str == "".to_string() { return None; }
	Some(str)
}

pub fn read_string() -> String {
	loop {
		if let Some(str) = read_string_or_empty() {
			break str;
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let mut g = analyze_file(args[1].clone());
	let mut gt = g.get_tree_mut();

	let mut end: bool = false;
	while !end {
		
		println!("Set time for move: {} {}", gt.get_move_number(), gt.get_move_text());
		let _min_str = read_string();
		if _min_str == "-" {
			break;
		}

		if let Ok(_min) = _min_str.parse::<u32>() {
			let hours = _min/60;
			let minutes = _min%60;
	
			let clock =
				hours.to_string() + ":" +
				if minutes < 10 { "0" } else { "" } +
				&minutes.to_string() +
				":00";
	
			gt.add_comment(
				comment::Comment::new_data(
					"".to_string(),
					vec![
						(comment::TagType::Clock, clock)
					]
				)
			);
	
			if gt.has_next_move() {
				gt = &mut *gt.get_next_move_mut().as_mut().unwrap();
			}
			else {
				end = true;
			}
		}
		else {
			println!("Could not parse number '{_min_str}'");
			break;
		}
	}

	let res = pgn_formatter::PgnFormatter::new()
		.set_print_comments(true)
		.set_print_variation(true)
		.set_print_result(true)
		.to_string(g.get_tree());

	println!("{res}");
	println!("");

}
