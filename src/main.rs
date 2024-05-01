mod game;
use game::Game;

fn main() {
    println!("Welcome to Hangman!");
    println!("This is a simple game, you likely know how it works.");
    println!("Simply, I will generate a random word. You must either guess the full word or individual letters of the word.");
    println!("1 letter wrong = 1 life     1 word guess wrong = 2 lifes\n");

    let game: Game = Game::new();
    game.start();
}
