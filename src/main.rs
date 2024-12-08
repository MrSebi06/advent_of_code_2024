use std::fs;

fn main() {
    let data = fs::read_to_string("src/data").expect("Cannot read the file");
    part_one(data.as_str());
    part_two();
}

fn part_one(mem: &str) {
    let mut i = 0;
    let mut res = 0;
    while let Some(pos) = mem[i..].find("mul(") {
        i += pos + 4;
        let x_str = mem[i..]
            .chars()
            .take(3)
            .take_while(|ch| ch.is_numeric())
            .collect::<String>();
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
        let y_str = mem[i..]
            .chars()
            .take(3)
            .take_while(|ch| ch.is_numeric())
            .collect::<String>();
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

fn part_two() {}
