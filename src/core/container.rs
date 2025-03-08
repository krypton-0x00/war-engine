#[derive(Debug)]
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
        self.cards
            .pop()
            .expect("[x] No card left on the pile cannot perform pop")
    }
    pub fn is_empty(&self) -> bool {
        if self.cards.len() <= 0 {
            return true;
        }
        false
    }
}
