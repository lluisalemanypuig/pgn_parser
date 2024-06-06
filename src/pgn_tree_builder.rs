
use crate::tokenizer;
use crate::game;
use crate::comment;

pub struct PGNTreeBuilder {
	keep_result: bool,
}

impl PGNTreeBuilder {
	pub fn new() -> PGNTreeBuilder {
		PGNTreeBuilder {
			keep_result: true
		}
	}
	
	pub fn set_keep_result(&mut self, use_res: bool) {
		self.keep_result = use_res;
	}
	
	fn parse_comment(&self, data: &tokenizer::TokenizedPGN, i: usize)
	-> (comment::Comment, usize)
	{
		(comment::Comment::new(), i + 1)
	}
	
	fn build_pgn_tree_rec(
		&self,
		data: &tokenizer::TokenizedPGN,
		mut i: usize
	)
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
					
					let (result, finished_variation, next) = self.build_pgn_tree_rec(&data, i + 1);
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
					
					let (com, next) = self.parse_comment(&data, i + 1);
					g.set_comment(com);
					i = next;
				},
				
				tokenizer::TokenType::VariantDelim { open: true } => {
					println!("Started a variation at {i}");
					
					let (res, _, next) = self.build_pgn_tree_rec(&data, i + 1);
					
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
					if self.keep_result {
						g.set_result(data[i].0.clone());
					}
					i += 1;
				},
				
				_ => {
					i += 1;
				}
			}
		}
		
		(Some(g), true, i + 1)
	}
	
	pub fn build_pgn_tree(&self, data: &tokenizer::TokenizedPGN)
	-> Option<game::Game>
	{
		let (res, _, _) = self.build_pgn_tree_rec(&data, 0);
		res
	}
}
