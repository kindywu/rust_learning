use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let sizes = [(9, 3, 3), (100, 10, 10), (25, 5, 5), (24, 4, 6), (18, 6, 3)];

    for &(len, rows, cols) in &sizes {
        let spiral = generate_spiral_array(len, rows, cols)?;
        print_spiral_array(&spiral);
    }

    Ok(())
}

fn generate_spiral_array(len: usize, rows: usize, cols: usize) -> Result<Vec<Vec<i32>>> {
    if len != rows * cols {
        return Err(anyhow!(
            "Array length {} must equal {} * {}",
            len,
            rows,
            cols
        ));
    }

    let mut array = vec![vec![0; cols]; rows];
    let (mut top, mut bottom, mut left, mut right) = (0, rows - 1, 0, cols - 1);
    let mut num = 1;

    while top <= bottom && left <= right {
        for i in left..=right {
            array[top][i] = num;
            num += 1;
        }
        top += 1;

        for i in top..=bottom {
            array[i][right] = num;
            num += 1;
        }
        right -= 1;

        if top <= bottom {
            for i in (left..=right).rev() {
                array[bottom][i] = num;
                num += 1;
            }
            bottom -= 1;
        }

        if left <= right {
            for i in (top..=bottom).rev() {
                array[i][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

    Ok(array)
}

fn print_spiral_array(array: &Vec<Vec<i32>>) {
    for row in array {
        for &val in row {
            print!("{:4}", val);
        }
        println!();
    }
}
