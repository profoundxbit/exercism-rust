#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut compare_result = Comparison::Unequal;

    if _first_list.is_empty() && !_second_list.is_empty() {
        compare_result = Comparison::Sublist;
    } else if !_first_list.is_empty() && _second_list.is_empty() {
        compare_result = Comparison::Superlist

    } else if _first_list.eq(_second_list) {
        compare_result = Comparison::Equal;
    } else {
        if _first_list.is_sublist_of(_second_list) {
            compare_result  = Comparison::Sublist
        } 
        if _second_list.is_sublist_of(_first_list) {
            compare_result = Comparison::Superlist
        }
    }
    compare_result
}

impl<T: std::cmp::PartialEq> List<T> for &[T] {
    fn is_sublist_of(&self,list :&[T]) -> bool {
        if self.is_empty() {
            return false
        }

        let window_length = self.len();
        for window in list.windows(window_length) {
            if *self == window {
                return true
            }
        }
        false
    }
}

trait List<T> {
    fn is_sublist_of(&self,list :&[T]) -> bool;
}
