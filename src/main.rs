use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/data").expect("Cannot read the file");
    let left: Vec<usize> = Regex::new(r"\W+").unwrap().split(contents.as_str()).enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, x)| x.parse::<usize>());
    let right: Vec<usize> = Regex::new(r"\W+").unwrap().split(contents.as_str()).enumerate().filter(|(i, _)| i % 2 == 1).map(|(_, x)| x.parse()).collect();
    let lr =  std::iter::zip(left, right);
    for (l, r) in lr {
        println!("{l}   {r}");
    }
}
