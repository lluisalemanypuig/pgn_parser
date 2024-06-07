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
	
	/* GETTERS */
	
	pub fn get_move_text(&self) -> &String {
		&self.game_move
	}
	pub fn get_move(&self) -> &Option<Box<Game>> {
		&self.main_line_next
	}
	pub fn empty_move(&self) -> bool {
		self.game_move == "".to_string()
	}
	pub fn get_variations(&self) -> &Vec<Game> {
		&self.variations
	}
}

pub fn print_game(g: &Game) {
	print!("{} ", g.get_move_text());
	if g.get_variations().len() > 0 {
		for var in g.get_variations().iter() {
			print!("( ");
			print_game(&var);
			print!(") ");
		}
	}
	if let Some(res) = g.get_move() {
		print_game(res);
	}
}
