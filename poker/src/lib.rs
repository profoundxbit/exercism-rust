use crate::hand::Hand;

mod card;
mod hand;
 
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut parsed_hands: Vec<Hand> = hands
        .iter()
        .map(Hand::from)
        .collect();
    parsed_hands
    .sort_by(|x, y| x.partial_cmp(y).unwrap());
    print!("Hands: {:?}", &parsed_hands);

    unimplemented!("Out of {:?}, which hand wins?", hands);
}
