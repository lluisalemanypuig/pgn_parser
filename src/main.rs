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
use std::io::BufRead;

mod tests;
mod comment;
mod game;
mod game_formatter;
mod tokenizer;
mod pgn_tree_builder;

fn analyze_file(p: String) {
	let mut entire_file_str = String::new();
	
	let file = std::fs::File::open(p).expect("Failed to open file");
	let reader = std::io::BufReader::new(file);
	for line in reader.lines() {
		entire_file_str.push_str( line.unwrap().trim() );
	}
	
	let (all_tokens, all_token_types) = tokenizer::tokenize(entire_file_str);
	
	let mut builder = pgn_tree_builder::PGNTreeBuilder::new();
	builder.set_token_list(all_tokens, all_token_types);

	if let Some(game) = builder.build_game_tree() {
		println!("{:#?}", game);
		
		let res = game_formatter::GameFormatter::new()
			.set_print_comments(true)
			.set_print_variation(true)
			.set_print_result(true)
			.to_string(&game);

		println!("{res}");
		println!("");
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	analyze_file(args[1].clone());
}
