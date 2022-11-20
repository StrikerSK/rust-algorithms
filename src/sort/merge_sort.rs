// MergeSort - recursively divide the array into two halves, sort each half and then merging them
pub fn sorting(param: Vec<i32>) {
    println!("Merge sort - input array: {:?}", param);
    println!("Merge sort - output array: {:?}", merge_sorter(param));
}

fn merge_sorter(param: Vec<i32>) -> Vec<i32> {
    return if param.len() < 2 {
        param
    } else {
        let middle = param.len() / 2;
        let left_array: Vec<i32> = merge_sorter(param[0..middle].to_vec());
        let right_array: Vec<i32> = merge_sorter(param[middle..param.len()].to_vec());
        merge_slices(left_array, right_array)
    }
}

fn merge_slices(slice1: Vec<i32>, slice2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < slice1.len() && j < slice2.len() {
        if slice1[i] < slice2[j] {
            result.push(slice1[i]);
            i = i + 1;
        } else {
            result.push(slice2[j]);
            j = j + 1;
        }
    }

    while i < slice1.len() {
        result.push(slice1[i]);
        i = i + 1;
    }

    while j < slice2.len() {
        result.push(slice2[j]);
        j = j + 1;
    }

    return result;
}