
use crate::tokenizer;
use crate::game;
use crate::comment;

pub struct PGNTreeBuilder {
	keep_result: bool,
	data: tokenizer::TokenizedPGN,
}

struct ParseResult {
	pub game: Option<game::Game>,
	pub comment: Option<comment::Comment>,
	pub finished_variation: bool,
	pub next: usize,
}

impl PGNTreeBuilder {
	pub fn new() -> PGNTreeBuilder {
		PGNTreeBuilder {
			keep_result: true,
			data: tokenizer::TokenizedPGN::new()
		}
	}
	
	pub fn set_keep_result(&mut self, use_res: bool) {
		self.keep_result = use_res;
	}
	
	pub fn set_data(&mut self, data: tokenizer::TokenizedPGN) {
		self.data = data;
	}
	
	fn parse_comment(&self, i: usize)
	-> (comment::Comment, usize)
	{
		(comment::Comment::new(), i + 1)
	}
	
	fn is_variant_or_comment(&self, i: usize) -> bool {
		match &self.data[i].1 {
			tokenizer::TokenType::VariantDelim { open: true } => true,
			tokenizer::TokenType::CommentDelim { open: true } => true,
			_ => false
		}
	}
	
	fn build_game_tree_rec(
		&self,
		mut i: usize,
		depth: usize
	)
	->	ParseResult
	{
		println!("Token {i}. Depth {depth}");
		
		if i == self.data.len() {
			return ParseResult {
				game: None,
				comment: None,
				finished_variation: false,
				next: self.data.len()
			};
		}
		
		if let tokenizer::TokenType::MoveNumber { id, side } = &self.data[i].1 {
			i += 1;
		}
		
		let mut g = game::Game::new();
		g.set_move_text(self.data[i].0.clone());
		i += 1;
		
		// read a series of variants or a comment.
		while i < self.data.len() && self.is_variant_or_comment(i) {
			
			match &self.data[i].1 {
				
				tokenizer::TokenType::VariantDelim { open: true } => {
					if let ParseResult {
						game: Some(gg),
						comment: None,
						finished_variation: f,
						next
					} = self.build_game_tree_rec(i + 1, depth + 1) {
						g.add_variation(gg);
						i = next;
					}
					else {
						panic!("Unexpected wrong return");
					}
				},
				
				tokenizer::TokenType::CommentDelim { open: true } => {
					let (comment, next) = self.parse_comment(i);
					g.set_comment(comment);
					i = next;
				},
				
				_ => { }
			}
		}
		
		if i < self.data.len() {
			
			if let tokenizer::TokenType::VariantDelim { open: false } = &self.data[i].1 {
				return ParseResult {
					game: Some(g),
					comment: None,
					finished_variation: true,
					next: i + 1
				};
			}
			
			if let ParseResult {
				game: Some(next),
				comment: None,
				finished_variation: _,
				next: _
			}
			= self.build_game_tree_rec(i, depth + 1) {
				
				g.set_next_move(next);
			}
		}
		
		ParseResult {
			game: Some(g),
			comment: None,
			finished_variation: false,
			next: i
		}
	}
	
	pub fn build_game_tree(&self) -> Option<game::Game> {
		let parse_result = self.build_game_tree_rec(0, 0);
		parse_result.game
	}
}
