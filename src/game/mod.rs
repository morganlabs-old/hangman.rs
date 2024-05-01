use rand::seq::SliceRandom;
use std::io;
use std::io::Write;

enum Difficulty {
    Easy,
    Normal,
    Hard,
}

const EASY_WORDS: [&str; 20] = [
    "Cat", "Dog", "Hat", "Sun", "Car", "Door", "Egg", "Bed", "Rain", "Toys", "Apple", "Ant", "Key",
    "Sock", "Leaf", "Ship", "Sand", "Girl", "Boy", "Bee",
];

const NORMAL_WORDS: [&str; 20] = [
    "Laptop", "Garden", "Pocket", "Bridge", "Wallet", "Rocket", "Helmet", "Camera", "Orange",
    "Mirror", "Pencil", "Guitar", "Jacket", "Flower", "Candle", "Puzzle", "Window", "Cheese",
    "Rabbit", "Piano",
];

const HARD_WORDS: [&str; 20] = [
    "Guileless",
    "Hegemony",
    "Impetuous",
    "Juxtapose",
    "Kowtow",
    "Lachrymose",
    "Munificent",
    "Nihilism",
    "Obfuscate",
    "Panacea",
    "Querulous",
    "Recalcitrant",
    "Sagacious",
    "Taciturn",
    "Ubiquity",
    "Vociferous",
    "Welter",
    "Xeric",
    "Yearn",
    "Zephyr",
];

const MAX_LIVES: u8 = 10;
const HANGMAN_ASCII: [&str; MAX_LIVES as usize] = [
    "








",
    "







______
",
    "

|
|
|
|
|
|
|_____
",
    "
_____
|
|
|
|
|
|
|_____
",
    "
_____
|    |
|    O
|
|
|
|
|_____
",
    "
_____
|    |
|    O
|    |
|
|
|
|_____
",
    "
_____
|    |
|    O
|   /|
|
|
|
|_____
",
    "
_____
|    |
|    O
|   /|\\
|
|
|
|_____
",
    "
_____
|    |
|    O
|   /|\\
|   /
|
|
|_____
",
    "
_____
|    |
|    O
|   /|\\
|   / \\
|
|
|_____
",
];

pub struct Game {
    word: String,
    correct_guesses: Vec<char>,
    incorrect_guesses: Vec<char>,
    lives_used: u8,
}

impl Game {
    pub fn new() -> Game {
        let difficulty = Self::get_difficulty();
        let word = Self::get_word(&difficulty);
        let correct_guesses = vec![' '; word.len()];

        return Game {
            word,
            correct_guesses,
            incorrect_guesses: vec![],
            lives_used: 1,
        };
    }

    fn get_word(difficulty: &Difficulty) -> String {
        let words_list = match difficulty {
            Difficulty::Easy => EASY_WORDS,
            Difficulty::Normal => NORMAL_WORDS,
            Difficulty::Hard => HARD_WORDS,
        };

        let word = words_list.choose(&mut rand::thread_rng());
        let word = match word {
            Some(w) => w,
            None => panic!("Failed to get word!"),
        };

        return word.to_string();
    }

    fn get_difficulty() -> Difficulty {
        loop {
            print!("What difficulty would you like to use? (Easy, Normal, Hard) ");
            io::stdout().flush().expect("Failed to flush the stdout.");
            let mut difficulty = String::new();
            io::stdin()
                .read_line(&mut difficulty)
                .expect("Failed to get user input.");
            let difficulty = difficulty.trim().to_string().to_lowercase();

            if difficulty.is_empty() {
                println!("Please input a value.");
                continue;
            }

            let difficulty = match difficulty.as_str() {
                "easy" => Difficulty::Easy,
                "normal" => Difficulty::Normal,
                "hard" => Difficulty::Hard,
                _ => {
                    println!("That difficulty is not valid.");
                    continue;
                }
            };

            return difficulty;
        }
    }

    pub fn start(&mut self) {
        loop {
            self.print_ui();

            let guess = self.get_guess("What's your guess?");
            self.check_guess(&guess);

            let is_game_over = self.is_game_over();
            match is_game_over {
                true => break,
                false => continue,
            };
        }
    }

    fn print_ui(&self) {
        self.clear_screen();
        let correct_guesses = &self
            .correct_guesses
            .iter()
            .map(|s| if *s == ' ' { '_' } else { *s })
            .collect::<String>();

        println!("Welcome to Hangman!");
        println!("This is a simple game, you likely know how it works.");
        println!("Simply, I will generate a random word. You must either guess the full word or individual letters of the word.");
        println!("1 letter wrong = 1 life | 1 word guess wrong = 2 lives\n");
        println!("{correct_guesses}");
        println!("{}/{}", self.lives_used, MAX_LIVES);
        println!("{}", self.get_hangman());
        println!(
            "Incorrect guesses:\n{}",
            self.incorrect_guesses
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    fn get_guess(&self, prompt: &str) -> String {
        loop {
            print!("{prompt} ");

            // Flush the stdout.
            // Without doing this, the prompt will appear after the user inputs
            // an answer.
            io::stdout().flush().expect("Failed to flush the stdout.");

            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to get user input.");
            let answer = answer.trim().to_string().to_lowercase();

            if answer.is_empty() {
                println!("Please input a value.");
                continue;
            } else if !answer.chars().all(char::is_alphabetic) {
                println!("Please only input letters.");
                continue;
            }

            if answer.len() == 1 {
                let answer = answer.chars().next().unwrap();
                if self.correct_guesses.contains(&answer)
                    || self.incorrect_guesses.contains(&answer)
                {
                    println!("You have already guessed this letter!");
                    continue;
                };
            };

            return answer;
        }
    }

    fn check_guess(&mut self, guess: &String) {
        if guess.len() == 1 {
            let guess = guess.chars().next().unwrap();
            self.check_letter_guess(&guess);
        } else {
            self.check_word_guess(&guess);
        };
    }

    fn check_letter_guess(&mut self, guess: &char) {
        if !self.word.contains(*guess) {
            self.incorrect_guesses.push(*guess);
            self.use_life(1);
            return;
        }

        for (idx, letter) in self.word.chars().enumerate() {
            if letter == *guess {
                self.correct_guesses[idx] = *guess;
            }
        }
    }

    fn check_word_guess(&mut self, guess: &String) {
        if self.word != *guess {
            self.use_life(2);
            return;
        }

        self.correct_guesses = guess.chars().collect();
    }

    fn use_life(&mut self, number_of_lives: u8) {
        self.lives_used += number_of_lives;
    }

    fn is_game_over(&mut self) -> bool {
        if self.lives_used >= MAX_LIVES {
            self.lives_used = MAX_LIVES;
            self.game_over();
            return true;
        } else if !self.correct_guesses.contains(&' ') {
            self.game_complete();
            return true;
        }

        return false;
    }

    fn game_over(&self) {
        self.print_ui();
        println!("Game over!");
        println!(
            "Unfortunately you didn't get the word! The word was \"{}\"",
            self.word
        );
    }

    fn game_complete(&self) {
        self.print_ui();
        println!("Game complete.");
    }

    fn clear_screen(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn get_hangman(&self) -> &str {
        return HANGMAN_ASCII[self.lives_used as usize - 1];
    }
}
