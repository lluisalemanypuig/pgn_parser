
pub struct Game {
    pub game_move: String,
    pub main_line: Option<Game>,
    pub variations: Vec<Game>,
    pub comments: Vec<String>
}

impl Game {
    pub fn new() -> Game {
        Game {
            game_move: "".to_string(),
            main_line: None,
            variations: Vec::new(),
            comments: Vec::new()
        }
    }
}