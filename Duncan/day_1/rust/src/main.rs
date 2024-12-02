use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_lists() -> Result<(Vec<i32>, Vec<i32>), String> {
    let file = File::open("input").map_err(|_| "Couldn't open input".to_string())?;
    let reader = BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| "Couldn't read line".to_string())?;
        let split: Vec<&str> = line.split_whitespace().collect();

        let left_value = split[0]
            .parse::<i32>()
            .map_err(|_| "Couldn't parse left value".to_string())?;
        let right_value: i32 = split[1]
            .parse::<i32>()
            .map_err(|_| "Couldn't parse right value".to_string())?;

        left.push(left_value);
        right.push(right_value);
    }

    left.sort();
    right.sort();

    Ok((left, right))
}

fn part_one(left: &Vec<i32>, right: &Vec<i32>) {
    let mut result = 0;

    for i in 0..left.len() {
        if left[i] > right[i] {
            result += left[i] - right[i];
        } else if right[i] > left[i] {
            result += right[i] - left[i];
        }
    }

    println!("Part one result: {}", result);
}

fn part_two(left: &Vec<i32>, right: &Vec<i32>) {
    let mut result = 0;

    for val in left {
        let right_count: i32 = right.iter().filter(|&n| *n == *val).count() as i32;
        result += *val * right_count;
    }

    println!("Part two result: {}", result);
}

fn main() {
    let (left, right) = match get_lists() {
        Ok(tup) => tup,
        Err(s) => {
            println!("Couldn't get lists: {}", s);
            return;
        }
    };

    part_one(&left, &right);
    part_two(&left, &right);
}
