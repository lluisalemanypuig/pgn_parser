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

	use pgn_parser::comment;
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

	fn make_game(file: String) -> game::GameTree {
		let mut builder = make_builder(file);
		if let Some(g) = builder.build_game_tree(0) {
			return g;
		}

		assert_eq!(true, false);
		game::GameTree::new()
	}

	fn result_move(res: &str) -> game::GameTree {
		game::GameTree::new_data(
			res.to_string(),
			true,
			0,
			None,
			vec![],
			None,
			vec![],
		)
	}

	fn make_comment(text: &str, tags: Vec<(&str,&str)>) -> comment::Comment {
		comment::Comment::new_data(
			text.to_string(),
			tags
			.iter()
			.map(
				|(a,b)| (
					comment::classify_tag(a.to_string()),
					b.to_string()
				)
			)
			.collect()
		)
	}

	// -------------------------------------------------

	fn make_game_0000() -> game::GameTree {
		game::GameTree::new_data(
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

	fn make_game_0001() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
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

	fn make_game_0002() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
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
				game::GameTree::new_data(
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

	fn make_game_0003() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
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
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::GameTree::new_data(
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

	fn make_game_0004() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
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
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					None,
					vec![],
				),
				game::GameTree::new_data(
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

	fn make_game_0005() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
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
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::GameTree::new_data(
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
				game::GameTree::new_data(
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

	fn make_game_0006() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
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
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::GameTree::new_data(
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
				game::GameTree::new_data(
					String::from("f4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![],
					Some(Box::new(
						game::GameTree::new_data(
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

	fn make_game_0007() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("c4"),
							false,
							2,
							Some(pgn_tokenizer::Side::White),
							vec![],
							Some(Box::new(
								game::GameTree::new_data(
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

	// -------------------------------------------------

	fn make_game_0008() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("c4"),
							false,
							2,
							Some(pgn_tokenizer::Side::White),
							vec![],
							Some(Box::new(
								game::GameTree::new_data(
									String::from("c6"),
									false,
									2,
									Some(pgn_tokenizer::Side::Black),
									vec![],
									None,
									vec![
										game::GameTree::new_data(
											String::from("e5"),
											false,
											2,
											Some(pgn_tokenizer::Side::Black),
											vec![],
											None,
											vec![],
										)
									],
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
	fn sample_0008() {
		assert_eq!(
			make_game("sample_games/0008.pgn".to_string()),
			make_game_0008()
		);
	}

	#[test]
	fn sample_0008_r() {
		let mut g = make_game_0008();
		g
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));

		assert_eq!(make_game("sample_games/0008-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0009() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![
				make_comment(
					"This is a bad move",
					vec![]
				)
			],
			None,
			vec![]
		)
	}

	#[test]
	fn sample_0009() {
		assert_eq!(
			make_game("sample_games/0009.pgn".to_string()),
			make_game_0009()
		);
	}

	#[test]
	fn sample_0009_r() {
		let mut g = make_game_0009();
		g
			.set_next_move(result_move("1-0"));

		assert_eq!(make_game("sample_games/0009-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0010() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![
				make_comment(
					"This is a bad move",
					vec![]
				)
			],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![
						make_comment(
							"This is also a bad move",
							vec![("%clk", "19")]
						)
					],
					None,
					vec![],
				)
			)),
			vec![]
		)
	}

	#[test]
	fn sample_0010() {
		assert_eq!(
			make_game("sample_games/0010.pgn".to_string()),
			make_game_0010()
		);
	}

	#[test]
	fn sample_0010_r() {
		let mut g = make_game_0010();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));

		assert_eq!(make_game("sample_games/0010-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0011() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![
				make_comment(
					"",
					vec![("%clk", "99")]
				)
			],
			Some(Box::new(
				game::GameTree::new_data(
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
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![
						make_comment(
							"",
							vec![("%clk", "99"), ("%eval", "-50")]
						)
					],
					None,
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0011() {
		assert_eq!(
			make_game("sample_games/0011.pgn".to_string()),
			make_game_0011()
		);
	}

	#[test]
	fn sample_0011_r() {
		let mut g = make_game_0011();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));

		assert_eq!(make_game("sample_games/0011-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0012() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![make_comment("Hola", vec![])],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![make_comment("大きい", vec![])],
					None,
					vec![],
				)
			)),
			vec![
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![make_comment("Adéu", vec![])],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("e5"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![make_comment("新しい", vec![])],
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
	fn sample_0012() {
		assert_eq!(
			make_game("sample_games/0012.pgn".to_string()),
			make_game_0012()
		);
	}

	#[test]
	fn sample_0012_r() {
		let mut g = make_game_0012();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1/2-1/2"));

		assert_eq!(make_game("sample_games/0012-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0013() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![make_comment("A A", vec![("%clk", "9")])],
			Some(Box::new(
				game::GameTree::new_data(
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
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![make_comment("B B", vec![("%clk", "9")])],
					None,
					vec![],
				),
				game::GameTree::new_data(
					String::from("f4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![make_comment("C C", vec![("%clk", "9")])],
					None,
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0013() {
		assert_eq!(
			make_game("sample_games/0013.pgn".to_string()),
			make_game_0013()
		);
	}

	#[test]
	fn sample_0013_r() {
		let mut g = make_game_0013();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1/2-1/2"));

		assert_eq!(make_game("sample_games/0013-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0014() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![make_comment("A A", vec![("%clk", "9"), ("%eval", "-9")])],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![make_comment("E E", vec![("%clk", "9"), ("%eval", "-9")])],
					None,
					vec![],
				)
			)),
			vec![
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![make_comment("B B", vec![("%clk", "9"), ("%eval", "-9")])],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("e5"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![make_comment("C C", vec![("%clk", "9"), ("%eval", "-9")])],
							None,
							vec![],
						),
					)),
					vec![],
				),
				game::GameTree::new_data(
					String::from("f4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![make_comment("D D", vec![("%clk", "9"), ("%eval", "-9")])],
					None,
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0014() {
		assert_eq!(
			make_game("sample_games/0014.pgn".to_string()),
			make_game_0014()
		);
	}

	#[test]
	fn sample_0014_r() {
		let mut g = make_game_0014();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));

		assert_eq!(make_game("sample_games/0014-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0015() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![make_comment("A A", vec![("%clk", "9"), ("%eval", "-9")])],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![make_comment("E E", vec![("%clk", "9"), ("%eval", "-9")])],
					None,
					vec![],
				)
			)),
			vec![
				game::GameTree::new_data(
					String::from("e4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![make_comment("B B", vec![("%clk", "9"), ("%eval", "-9")])],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("e5"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![make_comment("C C", vec![("%clk", "9"), ("%eval", "-9")])],
							None,
							vec![],
						),
					)),
					vec![],
				),
				game::GameTree::new_data(
					String::from("f4"),
					false,
					1,
					Some(pgn_tokenizer::Side::White),
					vec![make_comment("D D", vec![("%clk", "9"), ("%eval", "-9")])],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("Cc6"),
							false,
							1,
							Some(pgn_tokenizer::Side::Black),
							vec![],
							None,
							vec![],
						),
					)),
					vec![],
				)
			]
		)
	}

	#[test]
	fn sample_0015() {
		assert_eq!(
			make_game("sample_games/0015.pgn".to_string()),
			make_game_0015()
		);
	}

	#[test]
	fn sample_0015_r() {
		let mut g = make_game_0015();
		g
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1-0"));

		assert_eq!(make_game("sample_games/0015-r.pgn".to_string()), g);
	}
	
	// -------------------------------------------------

	fn make_game_0016() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![make_comment("B B", vec![])],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("c4"),
							false,
							2,
							Some(pgn_tokenizer::Side::White),
							vec![],
							Some(Box::new(
								game::GameTree::new_data(
									String::from("c6"),
									false,
									2,
									Some(pgn_tokenizer::Side::Black),
									vec![make_comment("C C", vec![])],
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
	fn sample_0016() {
		assert_eq!(
			make_game("sample_games/0016.pgn".to_string()),
			make_game_0016()
		);
	}

	#[test]
	fn sample_0016_r() {
		let mut g = make_game_0016();
		g
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("1-0"));

		assert_eq!(make_game("sample_games/0016-r.pgn".to_string()), g);
	}

	// -------------------------------------------------

	fn make_game_0017() -> game::GameTree {
		game::GameTree::new_data(
			String::from("d4"),
			false,
			1,
			Some(pgn_tokenizer::Side::White),
			vec![],
			Some(Box::new(
				game::GameTree::new_data(
					String::from("d5"),
					false,
					1,
					Some(pgn_tokenizer::Side::Black),
					vec![],
					Some(Box::new(
						game::GameTree::new_data(
							String::from("c4"),
							false,
							2,
							Some(pgn_tokenizer::Side::White),
							vec![],
							Some(Box::new(
								game::GameTree::new_data(
									String::from("c6"),
									false,
									2,
									Some(pgn_tokenizer::Side::Black),
									vec![],
									None,
									vec![
										game::GameTree::new_data(
											String::from("e5"),
											false,
											2,
											Some(pgn_tokenizer::Side::Black),
											vec![make_comment("P P", vec![])],
											None,
											vec![],
										)
									],
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
	fn sample_0017() {
		assert_eq!(
			make_game("sample_games/0017.pgn".to_string()),
			make_game_0017()
		);
	}

	#[test]
	fn sample_0017_r() {
		let mut g = make_game_0017();
		g
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.get_next_move_mut().as_mut().unwrap()
			.set_next_move(result_move("0-1"));

		assert_eq!(make_game("sample_games/0017-r.pgn".to_string()), g);
	}

}
 