use super::card::Card;

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }
}
