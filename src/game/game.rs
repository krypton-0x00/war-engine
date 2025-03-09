use super::player::Player;
use crate::core::{card::Card, container::Pile, deck::Deck};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Game {
    pub p1: Rc<RefCell<Player>>,
    pub p2: Rc<RefCell<Player>>,
    pub deck: Deck,

    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let p1 = Rc::new(RefCell::new(Player::new("PLAYER-1".to_string(), 1)));
        let p2 = Rc::new(RefCell::new(Player::new("PLAYER-2".to_string(), 2)));

        Self {
            p1,
            p2,
            game_over: false,
            deck,
        }
    }

    pub fn game_loop(&mut self) {
        self.draw();
        self.update();

        let mut temp_pile: Pile<Card> = Pile::new();

        while !self.game_over {
            let p1_card = self.p1.borrow_mut().show();
            let p2_card = self.p2.borrow_mut().show();

            if p1_card.is_none() || p2_card.is_none() {
                println!("Game Over! One player has no more cards.");
                self.game_over = true;
                break;
            }

            let p1_card = p1_card.unwrap();
            let p2_card = p2_card.unwrap();

            println!("Player 1: {},{}", p1_card.get_rank(), p1_card.get_suite());
            println!("Player 2: {},{}", p2_card.get_rank(), p2_card.get_suite());

            if p1_card.get_value() > p2_card.get_value() {
                println!("Player 1 Won the card.");
                self.p1.borrow_mut().cards.push(p2_card);
            } else if p1_card.get_value() < p2_card.get_value() {
                println!("Player 2 Won the card.");
                self.p2.borrow_mut().cards.push(p1_card);
            } else {
                println!("It's a tie!");
                temp_pile.push(p1_card);
                temp_pile.push(p2_card);
            }
        }
    }

    fn draw(&self) {
        clearscreen::clear().expect("[x] Error clearing the screen");
        println!("[+] STARTING....");
        println!("[+] Drawing Cards...");
    }
    fn update(&mut self) {
        self.deal()
    }

    fn deal(&mut self) {
        while !self.deck.cards.is_empty() {
            if let Some(card) = self.deck.cards.get(0).cloned() {
                self.p1.borrow_mut().cards.push(card);
                self.deck.cards.remove(0);
            }
            if let Some(card) = self.deck.cards.get(0).cloned() {
                self.p2.borrow_mut().cards.push(card);
                self.deck.cards.remove(0);
            }
        }
    }
}
