use std::mem;

use crate::hand::Hand;

mod card;
mod hand;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut parsed_hands: Vec<Hand> = hands.iter().map(Hand::from).collect();
    parsed_hands.sort_by(|x, y| x.partial_cmp(y).unwrap());


    // let raw_hand = parsed_hands.remove(parsed_hands.len()-1);
    let w_hand = parsed_hands.remove(parsed_hands.len()-1);
    let mut winning_hands: Vec<_> = parsed_hands.into_iter().filter(|x| x.eq(&w_hand)).map(|c| c.into_inner()).collect();
    winning_hands.push(w_hand.into_inner());
    winning_hands
    
    // unimplemented!("Out of {:?}, which hand wins?", hands);
}
