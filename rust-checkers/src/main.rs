mod board;
mod iohandler;
mod game;
pub use game::Game;
use std::env;

fn main() {
    // Set the stack trace on
    env::set_var("RUST_BACKTRACE", "1");
    // Initialize the game
    let mut g = Game::new();
    // Start the game loop
    g.play();
}