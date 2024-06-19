use anyhow::Result;
use std::{fmt::Debug, time::Instant};

fn main() -> Result<()> {
    let mut arr = [34, 7, 23, 32, 5, 62, 1, 31, 32, 5];
    // let mut arr = [2, 3, 4, 1];

    let high = arr.len() - 1;

    println!("{:20} {:?}", "Original array: ", arr);

    let start = Instant::now();
    quicksort(&mut arr, 0, high);
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    println!("{:20} {:?}", "Sorted array: ", arr);

    Ok(())
}

fn quicksort<T: Ord + Copy + Debug>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let sort = partition(arr, low, high);

        if sort > 1 {
            quicksort(arr, low, sort - 1);
        }
        quicksort(arr, sort + 1, high);
    }
}

fn partition<T: Ord + Copy + Debug>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = arr[high];

    let mut sort = low;

    for i in low..high {
        if arr[i] < pivot {
            arr.swap(i, sort);
            sort += 1;
        }
    }

    arr.swap(sort, high);

    // println!("{arr:?}  low {low} sort {sort} high {high}");

    sort
}
