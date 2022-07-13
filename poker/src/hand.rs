use std::{cmp::Ordering, collections::HashMap};

use crate::card::Card;

#[derive(Debug)]
pub struct Hand<'a> {
    cards: Vec<Card>,
    raw: &'a str,
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

impl Category {
    fn cmp_straight_flush(hand_cards : &[Card], other_hand_cards: &[Card]) -> Ordering {
        hand_cards[4].rank.cmp(&other_hand_cards[4].rank)
    }

    fn cmp_four_of_a_kind(hand_cards : &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut hand_cards_map = HashMap::new();
        let mut other_hand_cards_map = HashMap::new();
        for card in hand_cards {
            hand_cards_map.entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }
        for card in other_hand_cards {
            other_hand_cards_map.entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }

        let quad_rank_from_hand = hand_cards_map.iter().find(|(_key, &val)| val == 4);
        let quad_rank_from_other_hand = other_hand_cards_map.iter().find(|(_key, &val)| val == 4);
        let single_rank_from_hand = hand_cards_map.iter().find(|(_key, &val)| val == 1);
        let single_rank_from_other_hand = other_hand_cards_map.iter().find(|(_key, &val)| val == 1);

        match quad_rank_from_hand.cmp(&quad_rank_from_other_hand) {
            Ordering::Equal => single_rank_from_hand.cmp(&single_rank_from_other_hand),
            ordering => ordering
        }
    }
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

impl<'a> Hand<'a> {
    fn new(cards: Vec<Card>, raw: &'a str) -> Self {
        let mut cards = cards;
        let category = categorize_hand(&mut cards);
        Hand { cards, category, raw }
    }

    pub fn into_inner(self) -> &'a str {
        self.raw
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && 
        self.cards == other.cards
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.category.cmp(&other.category) {
            Ordering::Equal => same_category_cmp(self, other),
            result => result
        }
    }
}

fn same_category_cmp(hand: &Hand, other_hand: &Hand) -> Ordering {
    match &hand.category {
        Category::StraghtFlush => Category::cmp_straight_flush(&hand.cards, &other_hand.cards),
        Category::FourOfAKind => Category::cmp_four_of_a_kind(&hand.cards, &other_hand.cards),
        _ => panic!("Every hand should belong to a category!")
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> From<&&'a str> for Hand<'a> {
    fn from(str_hand: &&'a str) -> Self {
        let mut cards = str_hand
            .split_whitespace()
            .map(Card::from)
            .collect::<Vec<Card>>();

        cards.sort_unstable_by_key(|card| card.rank);

        Self::new(cards, *str_hand)
    }
}
