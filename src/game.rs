use crate::comment;
use crate::tokenizer;

#[derive(Debug)]
pub struct Game {
	m_game_move: String,
	m_is_result: bool,
	m_move_number: u32,
	m_side: Option<tokenizer::Side>,
	
	m_main_line_next: Option<Box<Game>>,
	m_variations: Vec<Game>,
	m_comment: comment::Comment,
}

impl Game {
	pub fn new() -> Game {
		Game {
			m_game_move: "".to_string(),
			m_is_result: false,
			m_move_number: 1,
			m_side: Some(tokenizer::Side::White),
			
			m_main_line_next: None,
			m_variations: Vec::new(),
			m_comment: comment::Comment::new(),
		}
	}
	
	pub fn set_move_text(
		&mut self,
		text: String,
		s: &tokenizer::Side,
		num: u32
	)
	{
		self.m_game_move = text;
		self.m_side = Some(s.clone());
		self.m_move_number = num;
	}

	pub fn set_result(&mut self, text: String) {
		self.m_game_move = text;
		self.m_is_result = true;
		self.m_side = None;
		self.m_move_number = 0;
	}

	pub fn set_next_move(&mut self, game: Game) {
		self.m_main_line_next = Some(Box::new(game));
	}

	pub fn add_variation(&mut self, variation: Game) {
		self.m_variations.push(variation);
	}

	pub fn set_comment(&mut self, comment: comment::Comment) {
		self.m_comment = comment;
	}
	
	/* GETTERS */
	
	pub fn get_move_text(&self) -> &String {
		&self.m_game_move
	}
	pub fn get_move(&self) -> &Option<Box<Game>> {
		&self.m_main_line_next
	}
	pub fn empty_move(&self) -> bool {
		self.m_game_move == "".to_string()
	}
	pub fn get_m_variations(&self) -> &Vec<Game> {
		&self.m_variations
	}
	
	pub fn to_string_rec(
		&self,
		show_move_number: bool
	)
	-> String
	{
		let mut s = String::new();
		
		if self.m_side == Some(tokenizer::Side::White) {
			s.push_str( &self.m_move_number.to_string() );
			s.push_str(". ");
		}
		else {
			
			if show_move_number {
				s.push_str( &self.m_move_number.to_string() );
				s.push_str("... ");
			}
		}
		
		s.push_str( &self.m_game_move.clone() );
		
		let exist_variations = self.m_variations.len() > 0;
		if exist_variations {
			for var in self.m_variations.iter() {
				s.push_str(" (");
				s.push_str(&var.to_string_rec( true ));
				s.push_str(")");
			}
		}
		if let Some(res) = &self.m_main_line_next {
			let next_side = tokenizer::other_side(&self.m_side.as_ref().unwrap());
			
			s.push_str(" ");
			s.push_str(&res.to_string_rec(exist_variations));
		}
		
		s
	}
	
	pub fn to_string(&self) -> String {
		self.to_string_rec( false )
	}
}

pub fn print_game(g: &Game) {
	print!("'{}'", g.to_string());
}
