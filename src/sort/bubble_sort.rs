// Compares value with rest of the array's values and swaps the value with the lowest value
pub fn sorting(mut param: Vec<i32>) {
    println!("Bubble sort - input array: {:?}", param);

    bubble_sorter(&mut param);
    println!("Bubble sort - output array: {:?}", param);
}

fn bubble_sorter(param: &mut Vec<i32>) {
    let mut is_swapped = false;

    for i in 0..(param.len() - 1) {
        is_swapped = false;

        for j in (i + 1)..(param.len()) {
            if param[i] > param[j] {
                let tmp_var1 = param[i];
                let tmp_var2 = param[j];
                param[i] = tmp_var2;
                param[j] = tmp_var1;
                is_swapped = true;
            }
        }

        if !is_swapped {
            break;
        }
    };
}
