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

fn is_straight_flush(cards: &[Card]) -> bool {
    cards.windows(2).all(is_sequential)
}

fn is_sequential(cards: &[Card]) -> bool {
    let c1_rank = cards.first().unwrap().rank as u8;
    let c2_rank = cards.last().unwrap().rank as u8;
    c1_rank + 1 == c2_rank
}

fn is_five_of_a_kind(cards: &Vec<Card>) -> bool {
    true
}

fn categorize_hand(cards: &mut [Card]) -> Category {
    cards.sort_unstable_by_key(|card| card.rank);
    if is_straight_flush(cards) {
        Category::StraghtFlush
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
            .map(Card::from)
            .collect::<Vec<Card>>();

        Self::new(cards)
    }
}
