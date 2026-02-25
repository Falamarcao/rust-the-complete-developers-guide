#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // Arrays have fixed lenght, and a slight performance improvement over use of vectors.
    let suits = ["Hearts", "Spades", "Diamonds"]; // let suits = vec!["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    let mut cards = vec![];

    // cards = vec![]; // with `mut` we can reassign the binding/variable;

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit); // format!("{value} of {suit}");
            cards.push(card); // with `mut` we can add values to the binding/variable.
        }
    }

    // let deck = Deck { cards: vec![] };
    // let deck = Deck { cards: Vec::new() };
    let deck = Deck { cards: cards }; // let deck = Deck { cards };

    println!("The deck: {deck:#?}"); // `:#?` format specifier to enable pretty-printing.
    // println!("The deck: {deck:?}");
    // println!("The deck: {:?}", deck);
}
