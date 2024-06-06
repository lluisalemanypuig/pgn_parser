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
	
	let res = tokenizer::tokenize(entire_file_str);
	for str in res.iter() {
		println!("{:?} -- {:?}", str.0, str.1);
	}
	
	let mut builder = pgn_tree_builder::PGNTreeBuilder::new();
	builder.set_keep_result(keep_result);
	if let Some(game) = builder.build_pgn_tree(&res) {
		println!("{:#?}", game);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	analyze_file(args[1].clone(), if args[2] == "true".to_string() { true } else { false });
}
