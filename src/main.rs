use std::fs;

fn main() {
    let data = fs::read_to_string("src/data").expect("Cannot read the file");
    part_one();
    part_two();
}

fn part_one() {}

fn part_two() {}