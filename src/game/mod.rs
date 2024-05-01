use std::io;
use std::io::Write;

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

    pub fn start(&self) {
        loop {
            let guess = self.get_input("What's your guess?");
            dbg!(guess);
        }
    }

    fn get_input(&self, prompt: &str) -> String {
        print!("{prompt} ");

        // Flush the stdout.
        // Without doing this, the prompt will appear after the user inputs
        // an answer.
        io::stdout().flush().expect("Failed to flush the stdout.");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to get user input.");

        return answer.trim().to_string();
    }
}
