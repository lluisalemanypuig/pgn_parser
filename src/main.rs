use std::env;
use std::io::{BufRead};

mod comment;
mod game;
mod tokenizer;

fn parse_comment(data: &tokenizer::TokenizedPGN, i: usize)
-> (comment::Comment, usize)
{
	(comment::Comment::new(), i + 1)
}

fn build_pgn_tree(data: &tokenizer::TokenizedPGN, mut i: usize)
->	(Option<game::Game>, bool, usize)
{
	let mut g = game::Game::new();
	let total_length = data.len();
	
	while i < total_length {
		println!("{i} -- {:#?}", data[i]);
		
		match &data[i].1 {
			tokenizer::TokenType::MoveNumber { id, side } => {
				i += 1;
			},
			
			tokenizer::TokenType::Text => {
				g.set_move_text(data[i].0.clone());
				
				let (result, finished_variation, next) = build_pgn_tree(&data, i + 1);
				if let Some(rest) = result {
					g.set_next_move(rest);
				}
				i = next;
				
				if finished_variation {
					return (Some(g), true, i);
				}
			},
			
			tokenizer::TokenType::CommentDelim { open: o } => {
				if *o == false {
					panic!("Unexpected closed comment delimiter at token {i}");
				}
				
				let (com, next) = parse_comment(&data, i + 1);
				g.set_comment(com);
				i = next;
			},
			
			tokenizer::TokenType::VariantDelim { open: true } => {
				println!("Started a variation at {i}");
				
				let (res, _, next) = build_pgn_tree(&data, i + 1);
				
				println!("Variation...");
				println!("{:#?}", res);
				
				if let Some(var) = res {
					println!("Add the variation to the game...");
					g.add_variation(var);
				}
				i = next;
			},
			
			tokenizer::TokenType::VariantDelim { open: false } => {
				println!("Finished a variation at {i}");
				return (None, true, i + 1);
			},
			
			tokenizer::TokenType::Result { result: _ } => {
				g.set_result(data[i].0.clone());
				i += 1;
			},
			
			_ => {
				i += 1;
			}
		}
	}
	
	(Some(g), true, i + 1)
}

fn analyze_file(p: String) {
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

	if let (Some(game), _, _) = build_pgn_tree(&res, 0) {
		println!("{:#?}", game);
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	analyze_file(args[1].clone());
}
