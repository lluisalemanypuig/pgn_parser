use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

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

// Set up the benchmark using Criterion's macros.
fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function(
		"Make a game from caro_kann_study_1",
		|b| {
			b.iter(|| make_game(black_box("benches/caro_kann_study.pgn".to_string())))
		}
	);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);