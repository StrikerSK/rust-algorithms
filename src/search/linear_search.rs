use crate::search::searching_trait::SearchingTrait;

pub struct LinearSearch;

impl SearchingTrait for LinearSearch {
    fn search(&self, input:&Vec<i32>, search_value: i32) -> i32 {
        searching(input, search_value) as i32
    }
}

pub fn searching(input:&Vec<i32>, search_value: i32) -> i32{
    let mut i: usize = 0;

    while i < input.len() {
        if input[i] == search_value {
            return i as i32;
        }
        i = i + 1;
    }

    return -1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // Example of searching that is not working on unsorted array
    fn test_unsorted_searching() {
        assert_eq!(searching(&vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54], 20), 8);
    }

    #[test]
    fn test_sorted_searching() {
        assert_eq!(searching(&vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123], 89), 8);
    }

    #[test]
    fn test_searching_nonexisting() {
        assert_eq!(searching(&vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123], 1000), -1);
    }

}