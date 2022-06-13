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
        compare_result = Comparison::Equal
    } else if _first_list.len() > _second_list.len() {
        // Comparison::Superlist
        let window_length = _second_list.len();
        while let Some(window) = _first_list.windows(window_length).next() {
            if window == _second_list {
                compare_result = Comparison::Superlist;
                break;
            }
        }
    } else {
        let window_length = _first_list.len();
        while let Some(window) = _second_list.windows(window_length).next() {
            if window == _first_list {
                compare_result = Comparison::Sublist;
                break;
            }
        }
    }

    compare_result
}
