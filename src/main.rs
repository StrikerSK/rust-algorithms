mod sort;
mod search;

fn main() {
    println!("-----------------------Sorting-----------------------");
    let input = vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54];
    sort::bubble_sort::sorting(input.clone());
    sort::merge_sort::sorting(input.clone());
    sort::quick_sort::sorting(input.clone());

    println!("-----------------------Searching-----------------------");
    search::linear_search::searching(input.clone(), 15);
}
