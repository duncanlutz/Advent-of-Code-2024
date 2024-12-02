use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    Increasing,
    Decreasing,
    None,
}

impl Direction {
    pub fn from_cmp(i: i32, j: i32) -> Self {
        if i > j {
            Self::Increasing
        } else if j > i {
            Self::Decreasing
        } else {
            Self::None
        }
    }
}

fn get_distance_and_direction(i: i32, j: i32) -> (i32, Direction) {
    let dir = Direction::from_cmp(i, j);
    let distance = match dir {
        Direction::None => 0i32,
        Direction::Increasing => i - j as i32,
        Direction::Decreasing => j - i as i32,
    };

    (distance, dir)
}

fn get_levels() -> Result<Vec<Vec<i32>>, String> {
    let file = File::open("input").map_err(|_| "Couldn't open input".to_string())?;
    let reader = BufReader::new(file);

    let mut levels: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| "Couldn't read line".to_string())?;
        let mut current_level = Vec::new();
        let split: Vec<&str> = line.split_whitespace().collect();

        for s in split {
            let num = s
                .parse::<i32>()
                .map_err(|_| "Couldn't parse split value".to_string())?;

            current_level.push(num);
        }

        levels.push(current_level);
    }

    Ok(levels)
}

fn part_one(levels: &Vec<Vec<i32>>) {
    let mut result = 0;

    'outer: for level in levels {
        let mut direction = Direction::Increasing;
        let mut last_number = 0;

        for i in 0..level.len() {
            let n = level[i];
            if i == 0 {
                last_number = n;
                continue;
            }

            let (distance, new_direction) = get_distance_and_direction(n, last_number);

            if i == 1 {
                direction = new_direction;
            } else if direction != new_direction {
                continue 'outer;
            }

            if distance < 1 || distance > 3 {
                continue 'outer;
            }

            last_number = n;
        }
        result += 1;
    }

    println!("Safe levels: {}", result);
}

fn get_unsafe_values(level: &Vec<i32>) -> Vec<usize> {
    let mut last_number = 0;
    let mut unsafe_values = Vec::new();
    let mut metrics: Vec<(i32, Direction)> = Vec::new();

    for i in 0..level.len() {
        let n = level[i];
        if i == 0 {
            last_number = n;
            continue;
        }

        metrics.push(get_distance_and_direction(n, last_number));

        last_number = n;
    }

    let mut increase_count = 0;
    let mut decrease_count = 0;
    let mut none_count = 0;

    for m in metrics.clone() {
        match m.1 {
            Direction::None => none_count += 1,
            Direction::Increasing => increase_count += 1,
            Direction::Decreasing => decrease_count += 1,
        }
    }

    let direction = if increase_count > decrease_count && increase_count > none_count {
        Direction::Increasing
    } else if decrease_count > increase_count && decrease_count > none_count {
        Direction::Decreasing
    } else {
        Direction::None
    };

    for i in 0..metrics.len() {
        let m = metrics[i].clone();
        if m.0 < 1 || m.0 > 3 || m.1 != direction {
            unsafe_values.push(i)
        }
    }

    return unsafe_values;
}

fn part_two(levels: &Vec<Vec<i32>>) {
    let mut result = 0;

    'outer: for level in levels {
        let mut level = level.clone();
        let unsafe_indexes = get_unsafe_values(&level);

        if unsafe_indexes.len() > 0 {
            'inner: for i in 0..level.len() {
                let mut l = level.clone();
                l.remove(i);

                let new_unsafe_indexes = get_unsafe_values(&l);

                if new_unsafe_indexes.len() > 0 {
                    if i == level.len() - 1 {
                        continue 'outer;
                    } else {
                        continue 'inner;
                    }
                } else {
                    level = l;
                    break 'inner;
                }
            }
        }

        result += 1;
    }

    println!("Safe levels: {}", result);
}

fn main() {
    let levels = match get_levels() {
        Ok(l) => l,
        Err(e) => {
            println!("Error getting levels: {}", e);
            return;
        }
    };

    part_one(&levels);
    part_two(&levels);
}
