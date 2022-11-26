pub fn calculate(value: i32) {
    println!("Fibonacci number for input: {} is {}", value, fibonacci_number(value));
}

fn fibonacci_number(value: i32) -> i32 {
    return if value == 0 {
        0
    } else if value == 1 {
        1
    } else {
        fibonacci_number(value - 1) + fibonacci_number(value - 2)
    }
}