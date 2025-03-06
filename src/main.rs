mod core;
use core::deck::Deck;
fn main() {
    let deck = Deck::new();

    let _ = &deck.cards.iter().for_each(|card| {
        println!(
            "SUITE: {:#?}, RANK:{}, COLOR:{:#?}, VALUE: {}",
            card.get_suite(),
            card.get_rank(),
            card.get_color(),
            card.get_value()
        );
    });
}
