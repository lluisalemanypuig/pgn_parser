
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
