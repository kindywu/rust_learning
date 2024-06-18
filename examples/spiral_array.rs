use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let spiral = spiral_array(9, 3, 3)?;

    print_spiral_array(spiral);

    let spiral = spiral_array(16, 4, 4)?;

    print_spiral_array(spiral);

    let spiral = spiral_array(25, 5, 5)?;

    print_spiral_array(spiral);

    let spiral = spiral_array(24, 4, 6)?;

    print_spiral_array(spiral);
    Ok(())
}

fn spiral_array(len: usize, rows: usize, cols: usize) -> Result<Vec<Vec<i32>>> {
    if len != (rows * cols) {
        return Err(anyhow!("array {} must equals {} * {}", len, rows, cols));
    }

    let mut spiral_array: Vec<Vec<i32>> = Vec::new();

    for _ in 0..rows {
        let mut col = Vec::new();
        for _ in 0..cols {
            col.push(0);
        }
        spiral_array.push(col);
    }

    let mut limit = Limit {
        right: cols - 1,
        down: rows - 1,
        left: 0,
        up: 0,
    };

    let mut direction = Direction::Right;
    let mut point = Point { x: 0, y: 0 };

    let mut i = 0;
    while i < len {
        let p = match direction {
            Direction::Right => {
                if point.y <= (limit.right as isize) {
                    spiral_array[point.x as usize][point.y as usize] = (i + 1) as i32;
                    point.y += 1;
                    Some(point)
                } else {
                    limit.up += 1;

                    point.x = limit.up as isize;

                    point.y = limit.right as isize;

                    direction = direction.next();

                    println!(
                        "point:{:?}, direction:{:?}, limit:{:?}",
                        &point, &direction, &limit
                    );
                    None
                }
            }
            Direction::Down => {
                if point.x <= (limit.down as isize) {
                    spiral_array[point.x as usize][point.y as usize] = (i + 1) as i32;
                    point.x += 1;
                    Some(point)
                } else {
                    limit.right -= 1;

                    point.x = limit.down as isize;

                    point.y = limit.right as isize;

                    direction = direction.next();

                    println!(
                        "point:{:?}, direction:{:?}, limit:{:?}",
                        &point, &direction, &limit
                    );
                    None
                }
            }
            Direction::Left => {
                if point.y >= (limit.left as isize) {
                    spiral_array[point.x as usize][point.y as usize] = (i + 1) as i32;
                    point.y -= 1;
                    Some(point)
                } else {
                    limit.down -= 1;

                    point.x = limit.down as isize;

                    point.y = limit.left as isize;

                    direction = direction.next();

                    println!(
                        "point:{:?}, direction:{:?}, limit:{:?}",
                        &point, &direction, &limit
                    );
                    None
                }
            }
            Direction::Up => {
                if point.x >= (limit.up as isize) {
                    spiral_array[point.x as usize][point.y as usize] = (i + 1) as i32;
                    point.x -= 1;
                    Some(point)
                } else {
                    limit.left += 1;

                    point.x = limit.up as isize;

                    point.y = limit.left as isize;

                    direction = direction.next();

                    println!(
                        "point:{:?}, direction:{:?}, limit:{:?}",
                        &point, &direction, &limit
                    );
                    None
                }
            }
        };

        match p {
            None => {}
            Some(_) => {
                i += 1;
            }
        };
    }

    Ok(spiral_array)
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Limit {
    right: usize,
    down: usize,
    left: usize,
    up: usize,
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

fn print_spiral_array(spiral_array: Vec<Vec<i32>>) {
    for row in spiral_array {
        for col in row {
            print!("{:5}", col);
        }
        println!()
    }
}
