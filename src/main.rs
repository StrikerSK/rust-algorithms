mod sort;

fn main() {
    let input = vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54];
    sort::bubble_sort::sorting(input.clone());
    sort::merge_sort::sorting(input.clone());
    sort::radix_sort::sorting(input.clone());
}
