#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: char,
}

#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Card {
    fn new(suit: Suit, rank: char) -> Self {
        Card { suit, rank }
    }
}

impl<'a> From<&'a str> for Card {
    fn from(str_card: &'a str) -> Self {
        println!("Attempting to parse {:?} into Card", &str_card);
        let rank = str_card
            .chars().next().unwrap();
        println!("{:?}", &rank);
        let suit = match str_card.chars().nth(1).unwrap() {
            'H' => Suit::Heart,
            'D' => Suit::Diamond,
            'S' => Suit::Spade,
            'C' => Suit::Club,
            _ => panic!("Unable to parse &str into card")
        };

        Card::new(suit, rank)
    }
}
