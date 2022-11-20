// QuickSort -
pub fn sorting(param: Vec<i32>) {
    println!("Quick sort - input array: {:?}", param);
    println!("Quick sort - output array: {:?}", quick_sorter(param.clone(), 0, param.len() - 1));
}

fn quick_sorter(mut slice: Vec<i32>, low: usize, high: usize) -> Vec<i32> {
    if low < high {;
        let p: usize = pivot_util(slice.clone(), low, high);
        quick_sorter(slice.clone(), low, p-1);
        quick_sorter(slice.clone(), p+1, high);
    }

    return slice;
}

fn pivot_util(mut slice: Vec<i32>, low: usize, high: usize) -> usize {
    let pivot: i32 = slice[high];
    let mut  i: usize = low;

    for j in low..high {
        if slice[j] < pivot {
            let tmp_var1: i32 = slice[i];
            let tmp_var2: i32 = slice[j];
            slice[i] = tmp_var2;
            slice[j] = tmp_var1;
            i = i + 1;
        }
    }

    let tmp_var1: i32 = slice[i];
    let tmp_var2: i32 = slice[high];
    slice[i] = tmp_var2;
    slice[high] = tmp_var1;
    return i;
}