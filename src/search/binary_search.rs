use crate::search::searching_trait::SearchingTrait;

pub struct BinarySearch;

impl SearchingTrait for BinarySearch {
    fn search(&self, input:&Vec<i32>, search_value: i32) -> i32 {
        binary_searching(&input.clone(), search_value) as i32
    }
}

fn binary_searching(input: &Vec<i32>, search_value: i32) -> i32 {
    let mut first_index: i32  = 0;
    let mut last_index: i32 = (input.len() - 1) as i32;
    let mut middle_index: i32 = (first_index + last_index) / 2;

    while first_index <= last_index {
        if input[middle_index as usize] < search_value {
            first_index = middle_index + 1;
        } else if input[middle_index as usize] == search_value {
            return middle_index;
        } else {
            last_index = middle_index - 1;
        }
        middle_index = (first_index + last_index) / 2;
    }

    return -1;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_unsorted_searching() {
        assert_eq!(binary_searching(&vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54], 20), -1);
    }

    #[test]
    fn test_sorted_searching() {
        assert_eq!(binary_searching(&vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123], 89), 8);
    }

    #[test]
    fn test_searching_nonexisting() {
        assert_eq!(binary_searching(&vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123], 1000), -1);
    }

}