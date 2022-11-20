// A utility function to get maximum value in arr[]
pub fn sorting(param: Vec<i32>) {
    println!("Radix sort - input array: {:?}", param);
    println!("Radix sort - output array: {:?}", radix_sort(param.clone(), param.len() as i32));
}

fn get_max_value(input: Vec<i32>, n: i32) -> i32 {
    let mut max_value = input[0];
    for i in 0..n {
        let current_value: i32 = input[i as usize];
        if max_value < current_value {
            max_value = current_value
        }
    }
    return max_value;
}

fn radix_sort(input:Vec<i32>, n: i32) {
    // Find the maximum number to know number of digits
    let m = get_max_value(input.clone(), n);

    // Do counting sort for every digit. Note that
    // instead of passing digit number, exp is passed.
    // exp is 10^i where i is current digit number
    let mut exp = 1;
    while  m / exp > 0 {
        count_sort(input.clone(), n, exp);
        exp = exp * 10;
    }
}

// A function to do counting sort of arr[] according to
// the digit represented by exp.
fn count_sort(mut input: Vec<i32>, n: i32, exp: i32) {
    let mut output = vec![n]; // output array
    let mut count = vec![10];

    // Store count of occurrences in count[]
    for i in 0..n {
        let mut  value = count[((input[i as usize] / exp) % 10) as usize];
        value = value + 1;
    }

    // Change count[i] so that count[i] now contains
    // actual position of this digit in output[]
    let mut i = 1;
    while i < 10 {
        count[i] = count[i] + count[i - 1];
        i = i + 1;
    }

    // Build the output array
    let mut i = n - 1;
    while i >= 0 {
        output[(count[((input[i as usize] / exp) % 10) as usize] - 1) as usize] = input[i as usize];

        let mut count_val = count[((input[i as usize] / exp) % 10) as usize];
        count_val = count_val - 1;
        i = i - 1;
    }

    // Copy the output array to arr[], so that arr[] now
    // contains sorted numbers according to current
    // digit
    for i in 0..n {
        input[i as usize] = output[i as usize];
    }
}
