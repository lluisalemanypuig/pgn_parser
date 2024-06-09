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

use std::env;
use std::io::{BufRead};

mod comment;
mod game;
mod tokenizer;
mod pgn_tree_builder;

fn analyze_file(p: String, keep_result: bool) {
	println!("Opening file: {p}");
	let mut entire_file_str = String::new();
	
	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		entire_file_str.push_str( line.unwrap().trim() );
	}
	
	let (all_tokens, all_token_types) = tokenizer::tokenize(entire_file_str);
	for i in 0..all_tokens.len() {
		println!("{i} :: {:?} -- {:?}", all_tokens[i], all_token_types[i]);
	}
	
	let mut builder = pgn_tree_builder::PGNTreeBuilder::new();
	
	builder.set_keep_result(keep_result);
	builder.set_token_list(all_tokens, all_token_types);
	
	if let Some(game) = builder.build_game_tree() {
		println!("{:#?}", game);
		game::print_game(&game);
		println!("");
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let keep = if args[2] == "true".to_string() { true } else { false };
	analyze_file(args[1].clone(), keep);
}
