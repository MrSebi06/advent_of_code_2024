use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/data").expect("Cannot read the file");
    let left: Vec<usize> = Regex::new(r"\W+").unwrap().split(contents.as_str()).enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, x)| x.parse().unwrap()).collect();
    let right: Vec<usize> = Regex::new(r"\W+").unwrap().split(contents.as_str()).enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, x)| x.parse().unwrap()).collect();

    part_one(left.clone(), right.clone());
    part_two(left, right);
}

fn part_one(mut left: Vec<usize>, mut right: Vec<usize>) {
    let mut sum = 0;

    let get_min_index = |x: &Vec<usize>| x.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).map(|(i, _)| i).unwrap();

    while right.len() != 0 {
        let min_left_index = get_min_index(&left);
        let min_right_index = get_min_index(&right);

        let min_left = left.remove(min_left_index);
        let min_right = right.remove(min_right_index);

        let distance = min_left.abs_diff(min_right);

        sum += distance;
    }
    println!("{sum}")
}

fn part_two(left: Vec<usize>, right: Vec<usize>) {
    let mut similarity = 0;
    for l in left {
        let factor = right.iter().filter(|&&r| r == l).count();
        similarity += l * factor;
    }
    println!("{similarity}")
}

