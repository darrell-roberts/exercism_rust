#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(sublist: &[T], superlist: &[T]) -> bool {
    sublist.is_empty() || superlist.windows(sublist.len()).any(|list| list == sublist)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if first_list.len() > second_list.len() && is_sublist(second_list, first_list) {
        Comparison::Superlist
    } else if first_list.len() < second_list.len() && is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
