// QuickSort -
pub fn sorting(mut param: Vec<i32>) {
    let length: usize = param.len() - 1;
    println!("Quick sort - input array: {:?}", param);

    quick_sorter(&mut param, 0, length);
    println!("Quick sort - output array: {:?}", param);
}

fn quick_sorter(slice: &mut Vec<i32>, low: usize, high: usize)  {
    if low < high {;
        let p: usize = pivot_util(slice, low, high);
        quick_sorter(slice, low, p-1);
        quick_sorter(slice, p+1, high);
    }
}

fn pivot_util(slice: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot: i32 = slice[high];
    let mut  i: usize = low;
    let mut  j: usize = low;

    while j < high {
        if slice[j] < pivot {
            let tmp_var1: i32 = slice[i];
            let tmp_var2: i32 = slice[j];
            slice[j] = tmp_var1;
            slice[i] = tmp_var2;
            i = i + 1;
        }
        j = j + 1;
    }

    let tmp_var1: i32 = slice[i];
    let tmp_var2: i32 = slice[high];
    slice[high] = tmp_var1;
    slice[i] = tmp_var2;
    return i;
}