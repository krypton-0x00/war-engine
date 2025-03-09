#[derive(Debug, Clone)]
pub struct Pile<Card> {
    pub cards: Vec<Card>,
}

impl<Card> Pile<Card> {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn pop(&mut self) -> Card {
        self.cards.remove(0)
    }
    pub fn is_empty(&self) -> bool {
        if self.cards.len() <= 0 {
            return true;
        }
        false
    }
}
