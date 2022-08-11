use std::{cmp::Ordering, collections::HashMap};

use crate::card::{Card, Rank};

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
    fn cmp_straight_flush(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        hand_cards[4].rank.cmp(&other_hand_cards[4].rank)
    }

    fn cmp_four_of_a_kind(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut hand_cards_map = HashMap::new();
        let mut other_hand_cards_map = HashMap::new();
        for card in hand_cards {
            hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }
        for card in other_hand_cards {
            other_hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }

        let quad_rank_from_hand = hand_cards_map.iter().find(|(_key, &val)| val == 4);
        let quad_rank_from_other_hand = other_hand_cards_map.iter().find(|(_key, &val)| val == 4);
        let single_rank_from_hand = hand_cards_map.iter().find(|(_key, &val)| val == 1);
        let single_rank_from_other_hand = other_hand_cards_map.iter().find(|(_key, &val)| val == 1);

        match quad_rank_from_hand.cmp(&quad_rank_from_other_hand) {
            Ordering::Equal => single_rank_from_hand.cmp(&single_rank_from_other_hand),
            ordering => ordering,
        }
    }

    fn cmp_three_of_a_kind(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut hand_cards_map = HashMap::new();
        let mut other_hand_cards_map = HashMap::new();
        for card in hand_cards {
            hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }
        for card in other_hand_cards {
            other_hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }

        let triple_rank_from_hand = hand_cards_map.iter().find(|(_key, &val)| val == 3);
        let triple_rank_from_other_hand = other_hand_cards_map.iter().find(|(_key, &val)| val == 3);

        let mut kickers_from_hand: Vec<_> = hand_cards_map
            .iter()
            .filter(|&x| x.0.ne(triple_rank_from_hand.unwrap().0))
            .map(|x| x.0)
            .collect();
        kickers_from_hand.sort();

        let mut kickers_from_other_hand: Vec<_> = other_hand_cards_map
            .iter()
            .filter(|&x| x.0.ne(triple_rank_from_other_hand.unwrap().0))
            .map(|x| x.0)
            .collect();
        kickers_from_other_hand.sort();

        match triple_rank_from_hand.cmp(&triple_rank_from_other_hand) {
            Ordering::Equal => match kickers_from_hand[1].cmp(kickers_from_other_hand[1]) {
                Ordering::Equal => kickers_from_hand[0].cmp(kickers_from_other_hand[0]),
                order => order,
            },
            ordering => ordering,
        }
    }

    fn cmp_two_pair(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut hand_cards_map = HashMap::new();
        let mut other_hand_cards_map = HashMap::new();
        for card in hand_cards {
            hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }
        for card in other_hand_cards {
            other_hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }

        let mut double_rank_from_hand = hand_cards_map
            .iter()
            .filter(|(_key, &val)| val == 2)
            .map(|x| x.0)
            .collect::<Vec<_>>();
        let mut double_rank_from_other_hand = other_hand_cards_map
            .iter()
            .filter(|(_key, &val)| val == 2)
            .map(|x| x.0)
            .collect::<Vec<_>>();
        double_rank_from_hand.sort();
        double_rank_from_other_hand.sort();

        let kicker_from_hand: Vec<_> = hand_cards_map
            .iter()
            .filter(|&x| !double_rank_from_hand.contains(&x.0))
            .map(|x| x.0)
            .collect();

        let kicker_from_other_hand: Vec<_> = other_hand_cards_map
            .iter()
            .filter(|&x| !double_rank_from_other_hand.contains(&x.0))
            .map(|x| x.0)
            .collect();

        match double_rank_from_hand[1].cmp(double_rank_from_other_hand[1]) {
            Ordering::Equal => match double_rank_from_hand[0].cmp(double_rank_from_other_hand[0]) {
                Ordering::Equal => kicker_from_hand[0].cmp(kicker_from_other_hand[0]),
                order => order,
            },
            ordering => ordering,
        }
    }

    fn cmp_pair(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut hand_cards_map = HashMap::new();
        let mut other_hand_cards_map = HashMap::new();
        for card in hand_cards {
            hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }
        for card in other_hand_cards {
            other_hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }

        let pair_from_hand = hand_cards_map.iter().find(|x| x.1 == &2).unwrap().0;
        let pair_from_other_hand = other_hand_cards_map.iter().find(|x| x.1 == &2).unwrap().0;

        let mut kickers_from_hand: Vec<_> = hand_cards_map
            .iter()
            .filter(|&x| x.0.ne(pair_from_hand))
            .map(|x| x.0)
            .collect();
        kickers_from_hand.sort();
        kickers_from_hand.reverse();

        let mut kickers_from_other_hand: Vec<_> = hand_cards_map
            .iter()
            .filter(|&x| x.0.ne(pair_from_other_hand))
            .map(|x| x.0)
            .collect();
        kickers_from_other_hand.sort();
        kickers_from_other_hand.reverse();

        match pair_from_hand.cmp(pair_from_other_hand) {
            Ordering::Equal => kickers_from_hand.cmp(&kickers_from_other_hand),
            o => o,
        }
    }

    fn cmp_full_house(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut hand_cards_map = HashMap::new();
        let mut other_hand_cards_map = HashMap::new();

        for card in hand_cards {
            hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }

        for card in other_hand_cards {
            other_hand_cards_map
                .entry(card.rank)
                .and_modify(|rank| *rank += 1)
                .or_insert(1);
        }

        let three_rank_from_hand = hand_cards_map.iter().find(|(_key, &val)| val == 3);
        let three_rank_from_other_hand = other_hand_cards_map.iter().find(|(_key, &val)| val == 3);
        let two_rank_from_hand = hand_cards_map.iter().find(|(_key, &val)| val == 2);
        let two_rank_from_other_hand = other_hand_cards_map.iter().find(|(_key, &val)| val == 2);

        match three_rank_from_hand.cmp(&three_rank_from_other_hand) {
            Ordering::Equal => two_rank_from_hand.cmp(&two_rank_from_other_hand),
            ordering => ordering,
        }
    }

    fn cmp_flush(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut i = hand_cards.len() - 1;
        while i > 0 {
            let x = hand_cards[i].rank;
            let y = other_hand_cards[i].rank;

            let result = x.cmp(&y);
            if result.is_ne() {
                return result;
            } else {
                i -= 1;
            }
        }
        Ordering::Equal
    }

    fn cmp_straight(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        // 2, 3, 4, 5, A
        if hand_cards[4].rank == Rank::Ace && hand_cards[3].rank == Rank::Five {
            Ordering::Less
        } else if other_hand_cards[4].rank == Rank::Ace && other_hand_cards[3].rank == Rank::Five {
            Ordering::Greater
        } else {
            Self::cmp_by_highest(hand_cards, other_hand_cards)
        }
    }

    fn cmp_by_highest(hand_cards: &[Card], other_hand_cards: &[Card]) -> Ordering {
        let mut hand_ranks = hand_cards.iter().map(|x| x.rank).collect::<Vec<_>>();
        hand_ranks.sort();
        hand_ranks.reverse();

        let mut other_hand_ranks = other_hand_cards.iter().map(|x| x.rank).collect::<Vec<_>>();
        other_hand_ranks.sort();
        other_hand_ranks.reverse();

        hand_ranks.cmp(&other_hand_ranks)
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

    let values: Vec<&i32> = map.values().collect();
    values.contains(&&3) && values.contains(&&2)
}

fn is_flush(cards: &[Card]) -> bool {
    let suit = &cards[0].suit;
    cards.iter().all(|card| card.suit == *suit)
}

fn is_straight(cards: &mut [Card]) -> bool {
    // Could be 10, J, Q, K, A
    // Could be 2, 3, 4, 5, A
    let result: bool;
    if cards[4].rank == Rank::Ace && cards[3].rank == Rank::Five {
        result = cards[..=3].windows(2).all(is_sequential);
    } else {
        result = cards.windows(2).all(is_sequential);
    }
    result
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
    let mut values: Vec<&i32> = map.values().collect();
    values.sort();
    values == vec![&1, &2, &2]
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
    cards.sort_by_key(|card| card.rank);
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
        Hand {
            cards,
            category,
            raw,
        }
    }

    pub fn into_inner(self) -> &'a str {
        self.raw
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.cards == other.cards
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.category.cmp(&other.category) {
            Ordering::Equal => same_category_cmp(self, other),
            result => result,
        }
    }
}

fn same_category_cmp(hand: &Hand, other_hand: &Hand) -> Ordering {
    match &hand.category {
        Category::StraghtFlush => Category::cmp_straight_flush(&hand.cards, &other_hand.cards),
        Category::FourOfAKind => Category::cmp_four_of_a_kind(&hand.cards, &other_hand.cards),
        Category::FullHouse => Category::cmp_full_house(&hand.cards, &other_hand.cards),
        Category::Flush => Category::cmp_flush(&hand.cards, &other_hand.cards),
        Category::Straight => Category::cmp_straight(&hand.cards, &other_hand.cards),
        Category::ThreeOfAKind => Category::cmp_three_of_a_kind(&hand.cards, &other_hand.cards),
        Category::TwoPair => Category::cmp_two_pair(&hand.cards, &other_hand.cards),
        Category::OnePair => Category::cmp_pair(&hand.cards, &other_hand.cards),
        Category::HighCard => Category::cmp_by_highest(&hand.cards, &other_hand.cards),
        _ => panic!("Every hand should belong to a category!"),
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> From<&&'a str> for Hand<'a> {
    fn from(str_hand: &&'a str) -> Self {
        let cards = str_hand
            .split_whitespace()
            .map(Card::from)
            .collect::<Vec<Card>>();

        Self::new(cards, *str_hand)
    }
}
