use super::card::{Card, Color, Suite};

#[derive(Debug, Clone)]
pub struct Deck {
    pub total: u8,
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![];

        let mut value = 2;
        for i in 2..54 {
            if cards.len() == 13 || cards.len() == 26 || cards.len() == 39 {
                value = 2
            }

            let rank: String = match value {
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                14 => "A".to_string(),
                x => x.to_string(),
            };
            let (suite, color) = match i {
                2..=15 => (Suite::SPADE, Color::BLACK),
                15..=27 => (Suite::CLUB, Color::BLACK),
                27..=40 => (Suite::HEART, Color::RED),
                40..=53 => (Suite::DIAMOND, Color::RED),
                //NOOB THING
                _ => panic!("Out of scope"),
            };

            let new_card = Card::new(rank, value, suite, color);
            cards.push(new_card);
            value += 1;
        }
        dbg!(&cards);
        Self {
            total: 52,
            cards: cards,
        }
    }
}
