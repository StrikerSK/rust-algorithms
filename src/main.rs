use std::borrow::Borrow;
use search::{binary_search, exponential_search, linear_search, searching_trait};

mod sort;
mod search;

fn main() {
    println!("-----------------------Sorting-----------------------");
    let unsorted_array = vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54];
    sort::bubble_sort::sorting(unsorted_array.clone());
    sort::merge_sort::sorting(unsorted_array.clone());
    sort::quick_sort::sorting(unsorted_array.clone());

    println!("-----------------------Searching-----------------------");
    let sorted_array: Vec<i32> = vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123];
    let searched_value: i32 = 35;

    println!("Searching - Input array: {:?}", sorted_array);
    println!("Searching - Searched value: {:?}", searched_value);

    print!("Binary search - ");
    searching_trait::search_value(&binary_search::BinarySearch, sorted_array.borrow(), searched_value);

    print!("\nExponential search - ");
    searching_trait::search_value(&exponential_search::ExponentialSearch, sorted_array.borrow(), searched_value);

    print!("\nLinear search - ");
    searching_trait::search_value(&linear_search::LinearSearch, sorted_array.borrow(), searched_value);
}
