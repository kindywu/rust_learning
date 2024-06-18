use std::{fmt::Debug, time::Instant};

fn main() {
    let mut arr = [34, 7, 23, 32, 5, 62, 1, 31, 32, 5];
    println!("Original array: {:?}", arr);
    let start = Instant::now();
    quicksort(&mut arr);
    let duration = start.elapsed();
    println!("Sorted array: {:?}", arr);
    println!("Time taken: {:?}", duration);
}

fn quicksort<T: Ord + Debug>(arr: &mut [T]) {
    if arr.len() > 1 {
        let pivot_index = partition(arr);
        // println!("array: {arr:?} pivot_index: {pivot_index}");
        let (left, right) = arr.split_at_mut(pivot_index);
        // println!("left: {left:?} right: {right:?}");
        quicksort(left);
        quicksort(&mut right[1..]);
    }
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut store_index = 0;
    for i in 0..arr.len() - 1 {
        if arr[i] < arr[arr.len() - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, arr.len() - 1);
    store_index
}
