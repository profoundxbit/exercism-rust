use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    category: Category,
}

#[derive(Debug)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraghtFlush,
    FiveOfAKind,
}

fn is_straight_flush(cards: &Vec<Card>) -> bool {
    cards
        .windows(2)
        .all(|pair| { 
            if pair.get(0).unwrap().rank + 1 == pair.get(1).unwrap().rank {
                true
            } else {
                false
            }
        })
}

fn is_five_of_a_kind(cards: &Vec<Card>) -> bool {
    true
}

fn categorize_hand(cards: &mut Vec<Card>) -> Category {
    cards.sort_unstable_by_key(|card| card.rank);
    if is_straight_flush(&cards) {
        Category::StraghtFlush
    } else if is_five_of_a_kind(&cards) {
        Category::FiveOfAKind
    } else {
        Category::HighCard
    }
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let mut cards = cards;
        let category = categorize_hand(&mut cards);
        Hand { cards, category }
    }
}

impl<'a> From<&&'a str> for Hand {
    fn from(str_hand: &&'a str) -> Self {
        let cards = str_hand
            .split_whitespace()
            .map(|str_card| -> Card { Card::from(str_card) })
            .collect::<Vec<Card>>();

        Self::new(cards)
    }
}
