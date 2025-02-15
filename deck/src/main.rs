use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of 'suits - 'hearts', 'sapdes'
        let suits = ["Hearts", "Spades", "Diamonds"];

        // List of 'values' - 'ace', 'two', 'three'
        let values = ["Ace", "Two", "Three"];

        // A vector to store cards
        let mut cards = vec![];

        // Double nested for loop
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    // Probably need to add error handling
    let cards = deck.deal(3);

    println!("Here's your hand: {:#?}", cards);
    println!("Here's your deck: {:#?}", deck);
}
