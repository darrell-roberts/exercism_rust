pub fn find<T: Ord>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut middle_index = array.len() / 2;
    let mut index = middle_index;
    let mut next_array = array;

    while !next_array.is_empty() {
        let middle_element = &next_array[middle_index];
        let (left, right) = next_array.split_at(middle_index);
        match key.cmp(middle_element) {
            std::cmp::Ordering::Less => {
                middle_index = left.len() / 2;
                index = middle_index;
                if left == next_array {
                    break;
                }
                next_array = left;
            }
            std::cmp::Ordering::Equal => return Some(index),
            std::cmp::Ordering::Greater => {
                middle_index = right.len() / 2;
                index += middle_index;
                if right == next_array {
                    break;
                }
                next_array = right;
            }
        }
    }

    None
}
