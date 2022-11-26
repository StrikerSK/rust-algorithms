mod sort;
mod other;

fn main() {
    let input = vec![56, 123, 2, 78, 15, 79, 35, 89, 20, 54];
    sort::bubble_sort::sorting(input.clone());
    sort::merge_sort::sorting(input.clone());
    sort::quick_sort::sorting(input.clone());

    // Fibonacci series is possible to run for values up to 45
    other::fibonacci_number::calculate(45);
}
