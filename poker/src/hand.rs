use std::{cmp::Ordering, collections::HashMap};

use crate::card::Card;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    category: Category,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    let suit = &cards[0].suit;

    cards.iter().all(|card| card.suit == *suit) && cards.windows(2).all(is_sequential)
}

fn is_sequential(cards: &[Card]) -> bool {
    let c1_rank = cards.first().unwrap().rank as u8;
    let c2_rank = cards.last().unwrap().rank as u8;
    c1_rank + 1 == c2_rank
}

fn is_four_of_a_kind(cards: &[Card]) -> bool {
    cards
        .windows(4)
        .any(|four_cards| four_cards.windows(2).all(|c| c[0].rank == c[1].rank))
}

fn is_full_house(cards: &[Card]) -> bool {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank)
            .and_modify(|rank| *rank += 1)
            .or_insert(1);
    }
    map.values().eq([3, 2].iter())
}

fn is_flush(cards: &[Card]) -> bool {
    let suit = &cards[0].suit;
    cards.iter().all(|card| card.suit == *suit)
}

fn is_straight(cards: &[Card]) -> bool {
    cards.windows(2).all(is_sequential)
}

fn is_three_of_a_kind(cards: &[Card]) -> bool {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank)
            .and_modify(|rank| *rank += 1)
            .or_insert(1);
    }
    map.values().any(|&x| x == 3)
}

fn is_two_pair(cards: &[Card]) -> bool {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank)
            .and_modify(|rank| *rank += 1)
            .or_insert(1);
    }
    map.values().eq([2, 2, 1].iter())
}

fn is_one_pair(cards: &[Card]) -> bool {
    let mut map = HashMap::new();
    for card in cards {
        map.entry(card.rank)
            .and_modify(|rank| *rank += 1)
            .or_insert(1);
    }
    map.values().any(|&x| x == 2)
}
fn categorize_hand(cards: &mut [Card]) -> Category {
    cards.sort_unstable_by_key(|card| card.rank);
    if is_straight_flush(cards) {
        Category::StraghtFlush
    } else if is_four_of_a_kind(cards) {
        Category::FourOfAKind
    } else if is_full_house(cards) {
        Category::FullHouse
    } else if is_flush(cards) {
        Category::Flush
    } else if is_straight(cards) {
        Category::Straight
    } else if is_three_of_a_kind(cards) {
        Category::ThreeOfAKind
    } else if is_two_pair(cards) {
        Category::TwoPair
    } else if is_one_pair(cards) {
        Category::OnePair
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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp_result = self.category.cmp(&other.category);
        match cmp_result {
            Ordering::Equal => panic!("Equal categories"),
            _ => Some(cmp_result),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category
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
