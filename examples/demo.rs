// cargo expand --example demo
#[tokio::main]
async fn main() {
    let mut data = vec![1, 2, 3];

    let last = data.last();

    // data.push(4);

    println!("last:{last:?}");

    data.push(4);
}
