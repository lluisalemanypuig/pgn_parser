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

	fn make_game(file: String) -> game::Game {
		let mut builder = make_builder(file);
		if let Some(g) = builder.build_game_tree() {
			return g;
		}

		assert_eq!(true, false);
		game::Game::new()
	}

	fn game_to_string(
		file: String,
		print_comments: bool,
		print_variations: bool,
		print_result: bool
	)
	-> String
	{
		game_formatter::GameFormatter::new()
			.set_print_comments(print_comments)
			.set_print_variation(print_variations)
			.set_print_result(print_result)
			.to_string( &make_game(file) )
	}

	#[test]
	fn sample_0000() {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0000.pgn".to_string(), pc, pv, pr),
				"1. d4".to_string()
			);
		}
		}
		}
	}
	#[test]
	fn sample_0000_r() {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0000-r.pgn".to_string(), pc, pv, true),
				"1. d4 1-0".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0000-r.pgn".to_string(), pc, pv, false),
				"1. d4".to_string()
			);
		}
		}
	}

	#[test]
	fn sample_0001() {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0001.pgn".to_string(), pc, pv, pr),
				"1. d4 d5".to_string()
			);
		}
		}
		}
	}
	#[test]
	fn sample_0001_r() {
		for pc in vec![false, true] {
		for pv in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0001-r.pgn".to_string(), pc, pv, true),
				"1. d4 d5 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0001-r.pgn".to_string(), pc, pv, false),
				"1. d4 d5".to_string()
			);
		}
		}
	}

	#[test]
	fn sample_0002() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0002.pgn".to_string(), pc, true, pr),
				"1. d4 (1. e4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0002.pgn".to_string(), pc, false, pr),
				"1. d4 d5".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0002_r() {
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), pc, true, true),
				"1. d4 (1. e4) 1... d5 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), pc, false, true),
				"1. d4 d5 0-1".to_string()
			);
			
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), pc, true, false),
				"1. d4 (1. e4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0002-r.pgn".to_string(), pc, false, false),
				"1. d4 d5".to_string()
			);
		}
	}

	#[test]
	fn sample_0003() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0003.pgn".to_string(), pc, true, pr),
				"1. d4 (1. e4 e5) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0003.pgn".to_string(), pc, false, pr),
				"1. d4 d5".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0003_r() {
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0003-r.pgn".to_string(), pc, true, true),
				"1. d4 (1. e4 e5) 1... d5 1/2-1/2".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0003-r.pgn".to_string(), pc, false, true),
				"1. d4 d5 1/2-1/2".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0003-r.pgn".to_string(), pc, true, false),
				"1. d4 (1. e4 e5) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0003-r.pgn".to_string(), pc, false, false),
				"1. d4 d5".to_string()
			);
		}
	}

	#[test]
	fn sample_0004() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0004.pgn".to_string(), pc, true, pr),
				"1. d4 (1. e4) (1. f4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0004.pgn".to_string(), pc, false, pr),
				"1. d4 d5".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0004_r() {
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0004-r.pgn".to_string(), pc, true, true),
				"1. d4 (1. e4) (1. f4) 1... d5 1/2-1/2".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0004-r.pgn".to_string(), pc, false, true),
				"1. d4 d5 1/2-1/2".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0004-r.pgn".to_string(), pc, true, false),
				"1. d4 (1. e4) (1. f4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0004-r.pgn".to_string(), pc, false, false),
				"1. d4 d5".to_string()
			);
		}
	}

	#[test]
	fn sample_0005() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0005.pgn".to_string(), pc, true, pr),
				"1. d4 (1. e4 e5) (1. f4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0005.pgn".to_string(), pc, false, pr),
				"1. d4 d5".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0005_r() {
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0005-r.pgn".to_string(), pc, true, true),
				"1. d4 (1. e4 e5) (1. f4) 1... d5 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0005-r.pgn".to_string(), pc, false, true),
				"1. d4 d5 0-1".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0005-r.pgn".to_string(), pc, true, false),
				"1. d4 (1. e4 e5) (1. f4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0005-r.pgn".to_string(), pc, false, false),
				"1. d4 d5".to_string()
			);
		}
	}

	#[test]
	fn sample_0006() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0006.pgn".to_string(), pc, true, pr),
				"1. d4 (1. e4 e5) (1. f4 Cc6) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0006.pgn".to_string(), pc, false, pr),
				"1. d4 d5".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0006_r() {
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0006-r.pgn".to_string(), pc, true, true),
				"1. d4 (1. e4 e5) (1. f4 Cc6) 1... d5 1-0".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0006-r.pgn".to_string(), pc, false, true),
				"1. d4 d5 1-0".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0006-r.pgn".to_string(), pc, true, false),
				"1. d4 (1. e4 e5) (1. f4 Cc6) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0006-r.pgn".to_string(), pc, false, false),
				"1. d4 d5".to_string()
			);
		}
	}

	#[test]
	fn sample_0007() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0007.pgn".to_string(), pc, true, pr),
				"1. d4 d5 2. c4 c6".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0007.pgn".to_string(), pc, false, pr),
				"1. d4 d5 2. c4 c6".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0007_r() {
		for pv in vec![false, true] {
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0007-r.pgn".to_string(), pc, pv, true),
				"1. d4 d5 2. c4 c6 1/2-1/2".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0007-r.pgn".to_string(), pc, pv, false),
				"1. d4 d5 2. c4 c6".to_string()
			);
		}
		}
	}

	#[test]
	fn sample_0008() {
		for pc in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0008.pgn".to_string(), pc, true, pr),
				"1. d4 d5 2. c4 c6 (2... e5)".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0008.pgn".to_string(), pc, false, pr),
				"1. d4 d5 2. c4 c6".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0008_r() {
		for pc in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0008-r.pgn".to_string(), pc, true, true),
				"1. d4 d5 2. c4 c6 (2... e5) 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0008-r.pgn".to_string(), pc, true, false),
				"1. d4 d5 2. c4 c6 (2... e5)".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0008-r.pgn".to_string(), pc, false, true),
				"1. d4 d5 2. c4 c6 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0008-r.pgn".to_string(), pc, false, false),
				"1. d4 d5 2. c4 c6".to_string()
			);
		}
	}

	/* ---------------------------------------------------------------------- */

	#[test]
	fn sample_0009() {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0009.pgn".to_string(), true, pv, pr),
				"1. d4 { This is a bad move }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0009.pgn".to_string(), false, pv, pr),
				"1. d4".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0009_r() {
		for pv in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0009-r.pgn".to_string(), true, pv, true),
				"1. d4 { This is a bad move } 1-0".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0009-r.pgn".to_string(), true, pv, false),
				"1. d4 { This is a bad move }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0009-r.pgn".to_string(), false, pv, true),
				"1. d4 1-0".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0009-r.pgn".to_string(), false, pv, false),
				"1. d4".to_string()
			);
		}
	}

	#[test]
	fn sample_0010() {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0010.pgn".to_string(), true, pv, pr),
				"1. d4 { This is a bad move } 1... d5 { [%clk 19] This is also a bad move }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0010.pgn".to_string(), false, pv, pr),
				"1. d4 d5".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0010_r() {
		for pv in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0010-r.pgn".to_string(), true, pv, true),
				"1. d4 { This is a bad move } 1... d5 { [%clk 19] This is also a bad move } 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0010-r.pgn".to_string(), true, pv, false),
				"1. d4 { This is a bad move } 1... d5 { [%clk 19] This is also a bad move }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0010-r.pgn".to_string(), false, pv, true),
				"1. d4 d5 0-1".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0010-r.pgn".to_string(), false, pv, false),
				"1. d4 d5".to_string()
			);
		}
	}

	#[test]
	fn sample_0011() {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0011.pgn".to_string(), true, true, pr),
				"1. d4 { [%clk 99] } (1. e4 { [%clk 99] [%eval -50] }) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0011.pgn".to_string(), true, false, pr),
				"1. d4 { [%clk 99] } 1... d5".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0011.pgn".to_string(), false, true, pr),
				"1. d4 (1. e4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0011.pgn".to_string(), false, false, pr),
				"1. d4 d5".to_string()
			);
		}
	}
	#[test]
	fn sample_0011_r() {
		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), true, true, true),
			"1. d4 { [%clk 99] } (1. e4 { [%clk 99] [%eval -50] }) 1... d5 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), true, true, false),
			"1. d4 { [%clk 99] } (1. e4 { [%clk 99] [%eval -50] }) 1... d5".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), true, false, true),
			"1. d4 { [%clk 99] } 1... d5 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), true, false, false),
			"1. d4 { [%clk 99] } 1... d5".to_string()
		);

		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), false, true, true),
			"1. d4 (1. e4) 1... d5 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), false, true, false),
			"1. d4 (1. e4) 1... d5".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), false, false, true),
			"1. d4 d5 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0011-r.pgn".to_string(), false, false, false),
			"1. d4 d5".to_string()
		);
	}

	#[test]
	fn sample_0012() {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0012.pgn".to_string(), true, true, pr),
				"1. d4 { Hola } (1. e4 { Adéu } 1... e5 { 新しい }) 1... d5 { 大きい }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0012.pgn".to_string(), true, false, pr),
				"1. d4 { Hola } 1... d5 { 大きい }".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0012.pgn".to_string(), false, true, pr),
				"1. d4 (1. e4 e5) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0012.pgn".to_string(), false, false, pr),
				"1. d4 d5".to_string()
			);
		}
	}
	#[test]
	fn sample_0012_r() {
		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), true, true, true),
			"1. d4 { Hola } (1. e4 { Adéu } 1... e5 { 新しい }) 1... d5 { 大きい } 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), true, true, false),
			"1. d4 { Hola } (1. e4 { Adéu } 1... e5 { 新しい }) 1... d5 { 大きい }".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), true, false, true),
			"1. d4 { Hola } 1... d5 { 大きい } 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), true, false, false),
			"1. d4 { Hola } 1... d5 { 大きい }".to_string()
		);

		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), false, true, true),
			"1. d4 (1. e4 e5) 1... d5 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), false, true, false),
			"1. d4 (1. e4 e5) 1... d5".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), false, false, true),
			"1. d4 d5 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0012-r.pgn".to_string(), false, false, false),
			"1. d4 d5".to_string()
		);
	}

	#[test]
	fn sample_0013() {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0013.pgn".to_string(), true, true, pr),
				"1. d4 { [%clk 9] A A } (1. e4 { [%clk 9] B B }) (1. f4 { [%clk 9] C C }) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0013.pgn".to_string(), true, false, pr),
				"1. d4 { [%clk 9] A A } 1... d5".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0013.pgn".to_string(), false, true, pr),
				"1. d4 (1. e4) (1. f4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0013.pgn".to_string(), false, false, pr),
				"1. d4 d5".to_string()
			);
		}
	}
	#[test]
	fn sample_0013_r() {
		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), true, true, true),
			"1. d4 { [%clk 9] A A } (1. e4 { [%clk 9] B B }) (1. f4 { [%clk 9] C C }) 1... d5 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), true, true, false),
			"1. d4 { [%clk 9] A A } (1. e4 { [%clk 9] B B }) (1. f4 { [%clk 9] C C }) 1... d5".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), true, false, true),
			"1. d4 { [%clk 9] A A } 1... d5 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), true, false, false),
			"1. d4 { [%clk 9] A A } 1... d5".to_string()
		);

		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), false, true, true),
			"1. d4 (1. e4) (1. f4) 1... d5 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), false, true, false),
			"1. d4 (1. e4) (1. f4) 1... d5".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), false, false, true),
			"1. d4 d5 1/2-1/2".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0013-r.pgn".to_string(), false, false, false),
			"1. d4 d5".to_string()
		);
	}

	#[test]
	fn sample_0014() {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0014.pgn".to_string(), true, true, pr),
				"1. d4 { [%clk 9] [%eval -9] A A } (1. e4 { [%clk 9] [%eval -9] B B } 1... e5 { [%clk 9] [%eval -9] C C }) (1. f4 { [%clk 9] [%eval -9] D D }) 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0014.pgn".to_string(), true, false, pr),
				"1. d4 { [%clk 9] [%eval -9] A A } 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0014.pgn".to_string(), false, true, pr),
				"1. d4 (1. e4 e5) (1. f4) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0014.pgn".to_string(), false, false, pr),
				"1. d4 d5".to_string()
			);
		}
	}
	#[test]
	fn sample_0014_r() {
		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), true, true, true),
			"1. d4 { [%clk 9] [%eval -9] A A } (1. e4 { [%clk 9] [%eval -9] B B } 1... e5 { [%clk 9] [%eval -9] C C }) (1. f4 { [%clk 9] [%eval -9] D D }) 1... d5 { [%clk 9] [%eval -9] E E } 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), true, true, false),
			"1. d4 { [%clk 9] [%eval -9] A A } (1. e4 { [%clk 9] [%eval -9] B B } 1... e5 { [%clk 9] [%eval -9] C C }) (1. f4 { [%clk 9] [%eval -9] D D }) 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), true, false, true),
			"1. d4 { [%clk 9] [%eval -9] A A } 1... d5 { [%clk 9] [%eval -9] E E } 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), true, false, false),
			"1. d4 { [%clk 9] [%eval -9] A A } 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
		);

		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), false, true, true),
			"1. d4 (1. e4 e5) (1. f4) 1... d5 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), false, true, false),
			"1. d4 (1. e4 e5) (1. f4) 1... d5".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), false, false, true),
			"1. d4 d5 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0014-r.pgn".to_string(), false, false, false),
			"1. d4 d5".to_string()
		);
	}

	#[test]
	fn sample_0015() {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0015.pgn".to_string(), true, true, pr),
				"1. d4 { [%clk 9] [%eval -9] A A } (1. e4 { [%clk 9] [%eval -9] B B } 1... e5 { [%clk 9] [%eval -9] C C }) (1. f4 { [%clk 9] [%eval -9] D D } 1... Cc6) 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0015.pgn".to_string(), true, false, pr),
				"1. d4 { [%clk 9] [%eval -9] A A } 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0015.pgn".to_string(), false, true, pr),
				"1. d4 (1. e4 e5) (1. f4 Cc6) 1... d5".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0015.pgn".to_string(), false, false, pr),
				"1. d4 d5".to_string()
			);
		}
	}
	#[test]
	fn sample_0015_r() {
		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), true, true, true),
			"1. d4 { [%clk 9] [%eval -9] A A } (1. e4 { [%clk 9] [%eval -9] B B } 1... e5 { [%clk 9] [%eval -9] C C }) (1. f4 { [%clk 9] [%eval -9] D D } 1... Cc6) 1... d5 { [%clk 9] [%eval -9] E E } 1-0".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), true, true, false),
			"1. d4 { [%clk 9] [%eval -9] A A } (1. e4 { [%clk 9] [%eval -9] B B } 1... e5 { [%clk 9] [%eval -9] C C }) (1. f4 { [%clk 9] [%eval -9] D D } 1... Cc6) 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), true, false, true),
			"1. d4 { [%clk 9] [%eval -9] A A } 1... d5 { [%clk 9] [%eval -9] E E } 1-0".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), true, false, false),
			"1. d4 { [%clk 9] [%eval -9] A A } 1... d5 { [%clk 9] [%eval -9] E E }".to_string()
		);

		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), false, true, true),
			"1. d4 (1. e4 e5) (1. f4 Cc6) 1... d5 1-0".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), false, true, false),
			"1. d4 (1. e4 e5) (1. f4 Cc6) 1... d5".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), false, false, true),
			"1. d4 d5 1-0".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0015-r.pgn".to_string(), false, false, false),
			"1. d4 d5".to_string()
		);
	}

	#[test]
	fn sample_0016() {
		for pv in vec![false, true] {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0016.pgn".to_string(), true, pv, pr),
				"1. d4 d5 { B B } 2. c4 c6 { C C }".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0016.pgn".to_string(), false, pv, pr),
				"1. d4 d5 2. c4 c6".to_string()
			);
		}
		}
	}
	#[test]
	fn sample_0016_r() {
		for pv in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0016-r.pgn".to_string(), true, pv, true),
				"1. d4 d5 { B B } 2. c4 c6 { C C } 1-0".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0016-r.pgn".to_string(), true, pv, false),
				"1. d4 d5 { B B } 2. c4 c6 { C C }".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0016-r.pgn".to_string(), false, pv, true),
				"1. d4 d5 2. c4 c6 1-0".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0016-r.pgn".to_string(), false, pv, false),
				"1. d4 d5 2. c4 c6".to_string()
			);
		}
	}

	#[test]
	fn sample_0017() {
		for pr in vec![false, true] {
			assert_eq!(
				game_to_string("sample_games/0017.pgn".to_string(), true, true, pr),
				"1. d4 d5 2. c4 c6 (2... e5 { P P })".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0017.pgn".to_string(), true, false, pr),
				"1. d4 d5 2. c4 c6".to_string()
			);

			assert_eq!(
				game_to_string("sample_games/0017.pgn".to_string(), false, true, pr),
				"1. d4 d5 2. c4 c6 (2... e5)".to_string()
			);
			assert_eq!(
				game_to_string("sample_games/0017.pgn".to_string(), false, false, pr),
				"1. d4 d5 2. c4 c6".to_string()
			);
		}
	}
	#[test]
	fn sample_0017_r() {
		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), true, true, true),
			"1. d4 d5 2. c4 c6 (2... e5 { P P }) 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), true, true, false),
			"1. d4 d5 2. c4 c6 (2... e5 { P P })".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), true, false, true),
			"1. d4 d5 2. c4 c6 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), true, false, false),
			"1. d4 d5 2. c4 c6".to_string()
		);

		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), false, true, true),
			"1. d4 d5 2. c4 c6 (2... e5) 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), false, true, false),
			"1. d4 d5 2. c4 c6 (2... e5)".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), false, false, true),
			"1. d4 d5 2. c4 c6 0-1".to_string()
		);
		assert_eq!(
			game_to_string("sample_games/0017-r.pgn".to_string(), false, false, false),
			"1. d4 d5 2. c4 c6".to_string()
		);
	}

}
