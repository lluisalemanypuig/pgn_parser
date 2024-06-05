
#[derive(Debug)]
pub struct Comment {
	pub text: String,
	pub tags: Vec<(String, String)>
}

impl Comment {
	pub fn new() -> Comment {
		Comment {
			text: String::new(),
			tags: Vec::new()
		}
	}
}

#[derive(Debug)]
pub struct Game {
	pub game_move: String,
	pub main_line_next: Option<Box<Game>>,
	pub variations: Vec<Game>,
	pub comment: Comment
}

impl Game {
	pub fn new() -> Game {
		Game {
			game_move: "".to_string(),
			main_line_next: None,
			variations: Vec::new(),
			comment: Comment::new()
		}
	}

	pub fn set_move_text(&mut self, text: String) {
		self.game_move = text;
	}

	pub fn set_next_move(&mut self, game: Game) {
		self.main_line_next = Some(Box::new(game));
	}

	pub fn add_variation(&mut self, variation: Game) {
		self.variations.push(variation);
	}

	pub fn set_comment(&mut self, comment: Comment) {
		self.comment = comment;
	}
}