mod sort;
mod search;

fn main() {
    println!("-----------------------Sorting-----------------------");
    let unsorted_array = vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54];
    sort::bubble_sort::sorting(unsorted_array.clone());
    sort::merge_sort::sorting(unsorted_array.clone());
    sort::quick_sort::sorting(unsorted_array.clone());

    println!("-----------------------Searching-----------------------");
    let sorted_array = vec![2, 15, 20, 35, 54, 56, 78, 79, 89, 123];
    let search_value = 35;
    search::linear_search::searching(&unsorted_array, search_value);
    search::exponential_search::searching(&sorted_array, search_value);
    search::binary_search::searching(&sorted_array, search_value);
}
