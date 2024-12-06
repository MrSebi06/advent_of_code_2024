use std::fs;

fn main() {
    let data = fs::read_to_string("src/data").expect("Cannot read the file");
    let reports: Vec<Vec<usize>> = data.split('\n').map(|x| x.split(' ').map(|y| y.parse().unwrap()).collect()).collect();

    part_one(&reports);
    part_two();
}

fn part_one(reports: &Vec<Vec<usize>>) {
    let mut safe_sum = 0;
    for report in reports {
        let safe = report.windows(2).all(|x| {
            x[0].abs_diff(x[1]) <= 3 && x[0].abs_diff(x[1]) >= 1
        });
        let is_decreasing = report.windows(2).all(|x| x[0] > x[1]);
        let is_increasing = report.windows(2).all(|x| x[0] < x[1]);
        safe_sum += (safe && (is_decreasing || is_increasing)) as usize;
    }
    println!("{safe_sum}")
}

fn part_two() {}