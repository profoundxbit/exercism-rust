use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl From<&str> for Rank {
    fn from(str_rank: &str) -> Self {
        match str_rank {
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" => Rank::Ten,
            "J" => Rank::Jack,
            "Q" => Rank::Queen,
            "K" => Rank::King,
            "A" => Rank::Ace,
            _ => panic!("Unable to process card rank")
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }
}

impl<'a> From<&'a str> for Card {
    fn from(str_card: &'a str) -> Self {
        println!("Attempting to parse {:?} into Card", &str_card);
        //Find index of suit
        //str before index of suit is rank

        let str_rank: &str = &str_card[.. str_card.len()-1];
        println!("{:?}", &str_rank);

        let suit = match str_card.chars().last().unwrap() {
            'H' => Suit::Heart,
            'D' => Suit::Diamond,
            'S' => Suit::Spade,
            'C' => Suit::Club,
            _ => panic!("Unable to parse &str into card")
        };

        Card::new(suit, str_rank.into())
    }
}
