mod core;
mod game;
use core::deck::Deck;

use game::game::Game;

fn main() {
    let mut game = Game::new();
    game.game_loop();
}
