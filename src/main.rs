use regex::bytes::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/data").expect("Cannot read the file");
    let byte_contents = contents.into_bytes();
    let left: Vec<&[u8]> = Regex::new(r"\W+").unwrap().split(&byte_contents).enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, x)| x).collect();
    for x in left {
        println!("{:?}", x);
    }
}
