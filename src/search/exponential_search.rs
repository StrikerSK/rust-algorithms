use std::borrow::Borrow;
use crate::search::searching_trait::SearchingTrait;

// This has same behaviour as binary search => input array must be sorted
pub struct ExponentialSearch;

impl SearchingTrait for ExponentialSearch {
    fn search(&self, input:&Vec<i32>, search_value: i32) -> i32 {
        exponential_searching(input.borrow(), input.len(), search_value) as i32
    }
}

fn exponential_searching(input: &Vec<i32>, n: usize, x: i32) -> i32 {
    if input[0] == x {
        return -1;
    }

    let mut bound: i32 = 1;

    while bound < n as i32 && input[bound as usize] <= x {
        bound = bound * 2;
    }

    return binary_searching(input, bound / 2, bound.min((n - 1) as i32), x);
}

fn binary_searching(input: &Vec<i32>, low: i32, high: i32, search_value: i32) -> i32 {
    if high >= low {
        let mid: i32 = (low + (high - low) / 2) as i32;

        if input[mid as usize] == search_value {
            return mid;
        }

        if input[mid as usize] > search_value {
            return binary_searching(input, low, mid - 1, search_value);
        }

        return binary_searching(input, mid + 1, high, search_value);
    }

    return -1;
}

#[cfg(test)]
mod tests {

    use std::borrow::Borrow;
    use super::*;

    #[test]
    // Example of searching that is not working on unsorted array
    fn test_unsorted_searching() {
        let input: Vec<i32> = vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54];
        assert_eq!(exponential_searching(input.borrow(), input.len(), 20), -1);
    }

    #[test]
    fn test_sorted_searching() {
        let input: Vec<i32> = vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123];
        assert_eq!(exponential_searching(input.borrow(), input.len(), 89), 8);
    }

    #[test]
    fn test_searching_nonexisting() {
        let input: Vec<i32> = vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123];
        assert_eq!(exponential_searching(input.borrow(), input.len(), 1000), -1);
    }

}