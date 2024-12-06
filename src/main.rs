use std::fs;

fn main() {
    let data = fs::read_to_string("src/data").expect("Cannot read the file");
    let reports: Vec<Vec<usize>> = data.split('\n').map(|x| x.split(' ').map(|y| y.parse().unwrap()).collect()).collect();

    part_one(&reports);
    part_two(&reports);
}

fn part_one(reports: &Vec<Vec<usize>>) {
    let mut safe_sum = 0;
    for report in reports {
        let safe = is_safe(&report);
        safe_sum += safe as usize;
    }
    println!("{safe_sum}")
}

fn part_two(reports: &Vec<Vec<usize>>) {
    let mut safe_sum = 0;
    for report in reports {
        let mut possible_reports: Vec<Vec<usize>> = report.iter().enumerate().map(|(i, _)| {
            let mut tmp = report.clone();
            tmp.remove(i);
            tmp
        }).collect();
        possible_reports.push(report.clone());
        let safe = possible_reports.iter().any(|x| is_safe(x));
        safe_sum += safe as usize;
    }
    println!("{safe_sum}")
}

fn is_safe(x: &Vec<usize>) -> bool {
    let safe_diff = x.windows(2).all(|x| {
        x[0].abs_diff(x[1]) <= 3 && x[0].abs_diff(x[1]) >= 1
    });
    let is_decreasing = x.windows(2).all(|x| x[0] > x[1]);
    let is_increasing = x.windows(2).all(|x| x[0] < x[1]);
    safe_diff && (is_increasing || is_decreasing)
}