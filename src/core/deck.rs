use super::card::{Card, Color, Suite};

#[derive(Debug, Clone)]
pub struct Deck {
    pub total: u8,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![];
        for i in 2..15 {
            let rank: String = match i {
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                x => x.to_string(),
            };
            let new_card = Card::new(rank, i, Suite::SPADE, Color::BLACK);
            cards.push(new_card);
        }
        for i in 2..15 {
            let rank: String = match i {
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                x => x.to_string(),
            };
            let new_card = Card::new(rank, i, Suite::CLUB, Color::BLACK);
            cards.push(new_card);
        }
        for i in 2..15 {
            let rank: String = match i {
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                x => x.to_string(),
            };
            let new_card = Card::new(rank, i, Suite::DIAMOND, Color::RED);
            cards.push(new_card);
        }
        for i in 2..15 {
            let rank: String = match i {
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                x => x.to_string(),
            };
            let new_card = Card::new(rank, i, Suite::HEART, Color::RED);
            cards.push(new_card);
        }
        Self {
            total: 52,
            cards: cards,
        }
    }
}
