use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // new() is a `Associated function`, tied to the struct definition
    fn new() -> Self {
        // fn new() -> Deck {

        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        
        // Alternative ways to return values

        // 1.
        // let deck = Deck { cards: cards };
        // return deck;

        // 1.1.
        // return Deck { cards: cards };
        // return Deck { cards };

        // 3. `Implicit return`
        // Deck { cards } // without a semicolon
        Self { cards } // without a semicolon
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        ) // `;` and no `return` keyword == `Implicit return`.
    }

}

fn main() {
    let mut deck = Deck::new();

    println!("The deck: {deck:#?}\n"); // `:#?` format specifier to enable pretty-printing.

    deck.shuffle(); // needs `mut`.
    println!("The shuffled deck: {:?}\n", deck);

    let cards = deck.deal(4);
    println!("Heres your hand: {:#?}\n", cards);

    // deck.deal(300); // panic: `attempt to subtract with overflow`, needs error handling or a nicer `deal()` method.
}
