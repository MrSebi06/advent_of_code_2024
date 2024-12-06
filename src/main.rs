use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/data").expect("Cannot read the file");
    let mut left: Vec<usize> = Regex::new(r"\W+").unwrap().split(contents.as_str()).enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, x)| x.parse().unwrap()).collect();
    let mut right: Vec<usize> = Regex::new(r"\W+").unwrap().split(contents.as_str()).enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, x)| x.parse().unwrap()).collect();

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

