pub fn searching(input:&Vec<i32>, search_value: i32) {
    println!("Linear search - input array: {:?}", input);
    println!("Linear search - search value: {:?}", search_value);

    let mut index: usize = 0;
    let mut i: usize = 0;

    while i < input.len() {
        if input[i] == search_value {
            index = i;
            break;
        }
        i = i + 1;
    }

    println!("Linear search - Value {:?} found at index: {:?}", search_value, index);
}