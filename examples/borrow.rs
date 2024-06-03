fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;

    println!(
        "addr of value:{:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    //addr of value:0x794f8ff968(0x794f8ff968), addr of data 0x794f8ffa08, data1: 0x794f8ff980

    println!("sum of data1:{}", sum(data1));
    //addr of value:0x794f8ff968, addr of ref 0x794f8ff828

    println!(
        "addr of items:[{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
    //addr of items:[0x1a9f3217850, 0x1a9f3217854, 0x1a9f3217858, 0x1a9f321785c]
}

fn sum(data: &Vec<u32>) -> u32 {
    println!("addr of value:{:p}, addr of ref {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}
