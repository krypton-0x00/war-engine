use super::player::Player;
use crate::core::deck::Deck;

#[derive(Debug)]
pub struct Game {
    pub p1: Player,
    pub p2: Player,
    pub deck: Deck,

    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let p1 = Player::new("PLAYER-1".to_string(), 1);
        let p2 = Player::new("PLAYER-2".to_string(), 2);

        Self {
            p1,
            p2,
            game_over: false,
            deck,
        }
    }

    pub fn game_loop(&mut self) {
        self.draw();
    }
    fn draw(&mut self) {
        clearscreen::clear().expect("[x] Error clearing the screen");
        println!("[+] STARTING....");
        println!("[+] Drawing Cards...");
        self.deal()
    }
    fn deal(&mut self) {
        // for i in 0..self.deck.cards.len() - 2 {
        //     self.p1.cards.push(self.deck.cards[i].clone());
        //     self.deck.cards.remove(0);
        //     self.p2.cards.push(self.deck.cards[i + 1].clone());
        //     self.deck.cards.remove(0);
        }
        println!(
            "{},{}",
            &self.p1.cards.cards.len(),
            &self.p1.cards.cards.len()
        );
        dbg!(&self.p1.cards);
        dbg!(&self.p2.cards);
    }
}
