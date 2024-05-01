#[derive(Debug)]
pub struct Game {
    word: String,
    correct_guesses: Vec<String>,
    incorrect_guesses: Vec<String>,
    lives_used: u8,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            word: String::from("word"),
            correct_guesses: vec![],
            incorrect_guesses: vec![],
            lives_used: 0,
        };
    }
}
