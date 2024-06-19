// cargo expand --example demo
#[tokio::main]
async fn main() {
    let mut arr = [34, 7, 23, 32, 5, 18, 1, 31, 32, 5];
    println!("Original array: {:?}", arr);

    let pivot_index = arr.len() / 2;

    let (left, right) = arr.split_at_mut(pivot_index);
    println!("{pivot_index}");
    println!("Left array: {:?}", left);
    println!("Right array: {:?}", &mut right[1..]);
}
