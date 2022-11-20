// This has same behaviour as binary search => input array must be sorted
pub fn searching(input:&Vec<i32>, search_value: i32) {
    println!("Exponential search - input array: {:?}", input);
    println!("Exponential search - search value: {:?}", search_value);

    let index: usize = exponential_searching(&input.clone(), input.len(),search_value);
    println!("Exponential search - Value {:?} found at index: {:?}", search_value, index);
}

fn exponential_searching(input: &Vec<i32>, n: usize, x: i32) -> usize {
    if input[0] == x {
        return 0;
    }

    let mut bound: usize = 1;
    let mut index: usize = 0;

    while bound < n && input[bound] <= x {
        bound = bound * 2;
    }

    index = binary_searching(input, bound / 2, bound.min((n - 1) as usize), x);
    return index;
}

fn binary_searching(input: &Vec<i32>, low: usize, high: usize, search_value: i32) -> usize {
    if high >= low {
        let mid: usize = low + (high - low) / 2;

        if input[mid] == search_value {
            return mid;
        }

        if input[mid] > search_value {
            return binary_searching(input, low, mid - 1, search_value);
        }

        return binary_searching(input, mid + 1, high, search_value);
    }

    return 0;
}