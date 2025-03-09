use crate::core::{card::Card, container::Pile};

#[derive(Debug, Clone)]
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
    pub fn show(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }

        let card = Some(self.cards.cards[0].clone());
        self.cards.pop();
        card
    }
}
