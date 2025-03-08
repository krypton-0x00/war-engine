use crate::core::{card::Card, container::Pile};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: u8,
    pub cards: Pile<Card>,
}

impl Player {
    pub fn new(name: String, id: u8) -> Self {
        let cards = Pile::new();
        Self { name, id, cards }
    }
    pub fn set_cards(&mut self, cards: Vec<Card>) {
        cards
            .iter()
            .for_each(|card| self.cards.push(card.to_owned()));
    }
    pub fn show(&self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }
        Some(self.cards.cards[0].clone())
    }
}
