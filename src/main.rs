use std::fs;

fn main() {
    let data = fs::read_to_string("src/data").expect("Cannot read the file");
    part_one(data.as_str());
    part_two(data.as_str());
}

fn part_one(mem: &str) {
    let mut i = 0;
    let mut res = 0;
    while let Some(pos) = mem[i..].find("mul(") {
        i += pos + 4;
        let x_str = get_char_str(&mem[i..]);
        let x: usize = match x_str.parse() {
            Ok(x) => {
                i += x_str.len();
                x
            }
            Err(_) => continue,
        };

        if mem.chars().nth(i).unwrap() != ',' {
            continue;
        }
        i += 1;

        let y_str = get_char_str(&mem[i..]);
        let y: usize = match y_str.parse() {
            Ok(y) => {
                i += y_str.len();
                y
            }
            Err(_) => continue,
        };

        if mem.chars().nth(i).unwrap() != ')' {
            continue;
        }

        res += x * y;
    }
    println!("{res}");
}

fn part_two(mem: &str) {
    let mut i = 0;
    let mut res = 0;
    let mut dont_index = mem[i..].find("don't()").unwrap();
    while let Some(pos) = mem[i..].find("mul(") {
        i += pos + 4;
        if i >= dont_index {
            i = match mem[i..].find("do()").ok_or(0) {
                Ok(index) => i + index,
                Err(_) => mem.len()
            };
            dont_index = i + mem[i..].find("don't()").unwrap_or_else(|| 0);
            continue;
        }
        let x_str = get_char_str(&mem[i..]);
        let x: usize = match x_str.parse() {
            Ok(x) => {
                i += x_str.len();
                x
            }
            Err(_) => continue,
        };

        if mem.chars().nth(i).unwrap() != ',' {
            continue;
        }
        i += 1;

        let y_str = get_char_str(&mem[i..]);
        let y: usize = match y_str.parse() {
            Ok(y) => {
                i += y_str.len();
                y
            }
            Err(_) => continue,
        };

        if mem.chars().nth(i).unwrap() != ')' {
            continue;
        }

        res += x * y;
    }
    println!("{res}");
}

fn get_char_str(x: &str) -> String {
    x.chars()
        .take(3)
        .take_while(|ch| ch.is_numeric())
        .collect::<String>()
}

