use std::borrow::Borrow;

pub trait SearchingTrait {
    fn search(&self, input:&Vec<i32>, search_value: i32) -> i32;
}

pub fn search_value(searcher: &dyn SearchingTrait, input_slice:&Vec<i32>, search_value: i32) {
    let index: i32 = searcher.search(input_slice.borrow(), search_value);
    print!("Value {:?} found at index: {:?}", search_value, index);
}