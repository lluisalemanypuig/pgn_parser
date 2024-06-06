use crate::comment;

#[derive(Debug)]
pub struct Game {
	pub game_move: String,
	is_result: bool,
	
	pub main_line_next: Option<Box<Game>>,
	pub variations: Vec<Game>,
	pub comment: comment::Comment
}

impl Game {
	pub fn new() -> Game {
		Game {
			game_move: "".to_string(),
			is_result: false,
			main_line_next: None,
			variations: Vec::new(),
			comment: comment::Comment::new()
		}
	}

	pub fn get_move(&self) -> &String {
		&self.game_move
	}
	pub fn empty_move(&self) -> bool {
		self.game_move == "".to_string()
	}

	pub fn set_move_text(&mut self, text: String) {
		self.game_move = text;
	}

	pub fn set_result(&mut self, text: String) {
		self.game_move = text;
		self.is_result = true;
	}

	pub fn set_next_move(&mut self, game: Game) {
		self.main_line_next = Some(Box::new(game));
	}

	pub fn add_variation(&mut self, variation: Game) {
		self.variations.push(variation);
	}

	pub fn set_comment(&mut self, comment: comment::Comment) {
		self.comment = comment;
	}
}
