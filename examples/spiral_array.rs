use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let sizes = [(9, 3, 3), (16, 4, 4), (25, 5, 5), (24, 4, 6), (18, 6, 3)];

    for (len, rows, cols) in sizes {
        let spiral = spiral_array(len, rows, cols)?;
        print_spiral_array(&spiral);
    }
    Ok(())
}

fn spiral_array(len: u32, rows: u32, cols: u32) -> Result<Vec<Vec<u32>>> {
    if len != rows * cols {
        return Err(anyhow!(
            "Array length {} must equal {} * {}",
            len,
            rows,
            cols
        ));
    }

    let mut spiral_array = vec![vec![0; cols as usize]; rows as usize];

    let mut limits = Limit {
        right: cols - 1,
        down: rows - 1,
        left: 0,
        up: 0,
    };

    let mut direction = Direction::Right;
    let mut point = Point { x: 0, y: 0 };
    let mut i: u32 = 0;

    while i < len {
        spiral_array[point.x as usize][point.y as usize] = i + 1;
        i += 1;

        match direction {
            Direction::Right => {
                if point.y < limits.right {
                    point.y += 1;
                } else {
                    limits.up += 1;
                    direction = direction.next();
                    point.x += 1;
                }
            }
            Direction::Down => {
                if point.x < limits.down {
                    point.x += 1;
                } else {
                    limits.right -= 1;
                    direction = direction.next();
                    point.y -= 1;
                }
            }
            Direction::Left => {
                if point.y > limits.left {
                    point.y -= 1;
                } else {
                    limits.down -= 1;
                    direction = direction.next();
                    point.x -= 1;
                }
            }
            Direction::Up => {
                if point.x > limits.up {
                    point.x -= 1;
                } else {
                    limits.left += 1;
                    direction = direction.next();
                    point.y += 1;
                }
            }
        }
    }

    Ok(spiral_array)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Limit {
    right: u32,
    down: u32,
    left: u32,
    up: u32,
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

fn print_spiral_array(spiral_array: &Vec<Vec<u32>>) {
    for row in spiral_array {
        for col in row {
            print!("{:5}", col);
        }
        println!()
    }
}
