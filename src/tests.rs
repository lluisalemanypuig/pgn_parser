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

#[cfg(test)]
mod tests {

	use std::io::BufRead;
	use crate::tokenizer;
	use crate::pgn_tree_builder;
	use crate::game;
	use crate::game_formatter;

	fn make_builder(file: String) -> pgn_tree_builder::PGNTreeBuilder {
		let mut entire_file_str = String::new();

		let file = std::fs::File::open(file).expect("Failed to open file");
		let reader = std::io::BufReader::new(file);
		for line in reader.lines() {
			entire_file_str.push_str( line.unwrap().trim() );
		}
		
		let (all_tokens, all_token_types) = tokenizer::tokenize(entire_file_str);
		
		let mut builder = pgn_tree_builder::PGNTreeBuilder::new();
		builder.set_token_list(all_tokens, all_token_types);

		builder
	}

	fn make_game(file: String, keep_result: bool) -> game::Game {
		let mut builder = make_builder(file);
		builder.set_keep_result(keep_result);
		if let Some(g) = builder.build_game_tree() {
			return g;
		}

		assert_eq!(true, false);
		game::Game::new()
	}

	fn game_to_string(
		file: String,
		keep_result: bool,
		print_comments: bool,
		print_variations: bool,
		print_result: bool
	)
	-> String
	{
		let g = make_game(file, keep_result);
		println!("{:#?}", g);

		game_formatter::GameFormatter::new()
			.set_print_comments(print_comments)
			.set_print_variation(print_variations)
			.set_print_result(print_result)
			.to_string( &g )
	}

	#[test]
	fn sample_0000() {
		for kr in vec![false, true] {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0000.pgn".to_string(), kr, pc, pv, pr),
				"1. d4".to_string()
			);
		}
		}
		}
		}
	}
	#[test]
	fn sample_0000_r() {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0000-r.pgn".to_string(), false, pc, pv, pr),
				"1. d4".to_string()
			);
		}
		}
		}
		for pc in vec![false, true] {
		for pv in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0000-r.pgn".to_string(), true, pc, pv, true),
				"1. d4 1-0".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0000-r.pgn".to_string(), true, pc, pv, false),
				"1. d4".to_string()
			);
		}
		}
	}

	#[test]
	fn sample_0001() {
		for kr in vec![false, true] {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0001.pgn".to_string(), kr, pc, pv, pr),
				"1. d4 d5".to_string()
			);
		}
		}
		}
		}
	}
	#[test]
	fn sample_0001_r() {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0001-r.pgn".to_string(), false, pc, pv, pr),
				"1. d4 d5".to_string()
			);
		}
		}
		}
		for pc in vec![false, true] {
		for pv in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0001-r.pgn".to_string(), true, pc, pv, true),
				"1. d4 d5 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0001-r.pgn".to_string(), true, pc, pv, false),
				"1. d4 d5".to_string()
			);
		}
		}
	}

	#[test]
	fn sample_0002() {
		for kr in vec![false, true] {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0002.pgn".to_string(), kr, pc, true, pr),
				"1. d4 (1. e4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0002.pgn".to_string(), kr, pc, false, pr),
				"1. d4 d5".to_string()
			);
		}
		}
		}
	}
	#[test]
	fn sample_0002_r() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), false, pc, true, pr),
				"1. d4 (1. e4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), false, pc, false, pr),
				"1. d4 d5".to_string()
			);
		}
		}
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), true, pc, true, true),
				"1. d4 (1. e4) 1... d5 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), true, pc, false, false),
				"1. d4 d5".to_string()
			);
		}
	}

}
