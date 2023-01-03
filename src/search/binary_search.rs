pub fn searching(input:&Vec<i32>, search_value: i32) {
    println!("Binary search - input array: {:?}", input);
    println!("Binary search - search value: {:?}", search_value);

    let index: usize = binary_searching(&input.clone(), search_value);
    println!("Binary search - Value {:?} found at index: {:?}", search_value, index);
}

fn binary_searching(input: &Vec<i32>, search_value: i32) -> usize {
    let mut first_index: usize = 0;
    let mut last_index: usize = input.len() - 1;
    let mut middle_index: usize = (first_index + last_index) / 2;

    while first_index <= last_index {
        if input[middle_index] < search_value {
            first_index = middle_index + 1;
        } else if input[middle_index] == search_value {
            return middle_index;
        } else {
            last_index = middle_index - 1;
        }
        middle_index = (first_index + last_index) / 2;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_unsorted_searching() {
        assert_eq!(binary_searching(&vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54], 89), 7);
    }

    #[test]
    fn test_sorted_searching() {
        assert_eq!(binary_searching(&vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123], 89), 8);
    }

}