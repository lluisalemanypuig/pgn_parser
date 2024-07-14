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

	use pgn_parser::game;
	use pgn_parser::pgn_tree_builder;
	use pgn_parser::pgn_tokenizer;

	use std::io::BufRead;

	fn make_builder(file: String) -> pgn_tree_builder::PGNTreeBuilder {
		let mut entire_file_str = String::new();

		let file = std::fs::File::open(file).expect("Failed to open file");
		let reader = std::io::BufReader::new(file);
		for line in reader.lines() {
			entire_file_str.push_str( line.unwrap().trim() );
		}
		
		let (all_tokens, all_token_types) = pgn_tokenizer::tokenize(entire_file_str);
		
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

	fn result_move(res: &str) -> game::Game {
		game::Game::new_data(
			res.to_string(),
			true,
			0,
			None,
			vec![],
			None,
			vec![],
		)
	}

	// -------------------------------------------------

	fn make_game_0000() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			None,
			vec![]
		)
	}

	#[test]
	fn sample_0000() {
		assert_eq!(
			make_game("sample_games/0000.pgn".to_string()),
			make_game_0000()
		);
	}

	#[test]
	fn sample_0000_r() {
		let mut g = make_game_0000();
		g
			.set_next_move(result_move("1-0"));
		assert_eq!(make_game("sample_games/0000-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0001() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::Game::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					None,
					vec![],
				)
			)),
			vec![]
		)
	}

	#[test]
	fn sample_0001() {
		assert_eq!(
			make_game("sample_games/0001.pgn".to_string()),
			make_game_0001()
		);
	}

	#[test]
	fn sample_0001_r() {
		let mut g = make_game_0001();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));
		assert_eq!(make_game("sample_games/0001-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0002() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::Game::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					None,
					vec![],
				)
			)),
			vec![
				game::Game::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					None,
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0002() {
		assert_eq!(
			make_game("sample_games/0002.pgn".to_string()),
			make_game_0002()
		);
	}

	#[test]
	fn sample_0002_r() {
		let mut g = make_game_0002();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));
		assert_eq!(make_game("sample_games/0002-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0003() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::Game::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					None,
					vec![],
				)
			)),
			vec![
				game::Game::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::Game::new_data(
							String::from("e5"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![],
							None,
							vec![],
						)
					)),
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0003() {
		assert_eq!(
			make_game("sample_games/0003.pgn".to_string()),
			make_game_0003()
		);
	}

	#[test]
	fn sample_0003_r() {
		let mut g = make_game_0003();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1/2-1/2"));
		assert_eq!(make_game("sample_games/0003-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0004() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::Game::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					None,
					vec![],
				)
			)),
			vec![
				game::Game::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					None,
					vec![],
				),
				game::Game::new_data(
					String::from("f4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					None,
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0004() {
		assert_eq!(
			make_game("sample_games/0004.pgn".to_string()),
			make_game_0004()
		);
	}

	#[test]
	fn sample_0004_r() {
		let mut g = make_game_0004();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1/2-1/2"));
		assert_eq!(make_game("sample_games/0004-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0005() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::Game::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					None,
					vec![],
				)
			)),
			vec![
				game::Game::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::Game::new_data(
							String::from("e5"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![],
							None,
							vec![],
						)
					)),
					vec![],
				),
				game::Game::new_data(
					String::from("f4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					None,
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0005() {
		assert_eq!(
			make_game("sample_games/0005.pgn".to_string()),
			make_game_0005()
		);
	}

	#[test]
	fn sample_0005_r() {
		let mut g = make_game_0005();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));
		assert_eq!(make_game("sample_games/0005-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0006() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::Game::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					None,
					vec![],
				)
			)),
			vec![
				game::Game::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::Game::new_data(
							String::from("e5"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![],
							None,
							vec![],
						)
					)),
					vec![],
				),
				game::Game::new_data(
					String::from("f4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::Game::new_data(
							String::from("Cc6"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![],
							None,
							vec![],
						)
					)),
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0006() {
		assert_eq!(
			make_game("sample_games/0006.pgn".to_string()),
			make_game_0006()
		);
	}

	#[test]
	fn sample_0006_r() {
		let mut g = make_game_0006();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1-0"));

		assert_eq!(make_game("sample_games/0006-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0007() -> game::Game {
		game::Game::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::Game::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					Some(Box::new(
						game::Game::new_data(
							String::from("c4"),
							false,
							2,
							Some(pgn_tokenizer::Side::White),
							vec![],
							Some(Box::new(
								game::Game::new_data(
									String::from("c6"),
									false,
									2,
									Some(pgn_tokenizer::Side::Black),
									vec![],
									None,
									vec![],
								)
							)),
							vec![],
						)
					)),
					vec![],
				)
			)),
			vec![]
		)
	}

	#[test]
	fn sample_0007() {
		assert_eq!(
			make_game("sample_games/0007.pgn".to_string()),
			make_game_0007()
		);
	}

	#[test]
	fn sample_0007_r() {
		let mut g = make_game_0007();
		g
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1/2-1/2"));

		assert_eq!(make_game("sample_games/0007-r.pgn".to_string()), g);
	}


}
 