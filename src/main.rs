use std::fs;
use struct_iterable::Iterable;

fn main() {
    let data = fs::read_to_string("src/data").expect("Cannot read the file");
    part_one(data.as_str());
    part_two();
}

#[derive(Iterable)]
struct Directions {
    N: bool,
    NE: bool,
    E: bool,
    SE: bool,
    S: bool,
    SW: bool,
    W: bool,
    NW: bool,
}
fn part_one(grid: &str) {
    let pat = ['M', 'A', 'S'];
    let mut i = 0;
    let line_size = grid.find('\n').unwrap();
    let mut total = 0;
    while let Some(pos) = grid.find('X') {
        i += pos;
        let mut dirs = Directions {
            N: true,
            NE: true,
            E: true,
            SE: true,
            S: true,
            SW: true,
            W: true,
            NW: true,
        };
        for (j, c) in pat.iter().enumerate() {
            check_dirs(grid, i, *c, j, &mut dirs, line_size);
        }
        let n = count_true_directions(&dirs);
        total += n;
    }
    println!("{total}")
}

fn count_true_directions(dirs: &Directions) -> usize {
    [
        dirs.N, dirs.NE, dirs.E, dirs.SE, dirs.S, dirs.SW, dirs.W, dirs.NW,
    ]
        .iter()
        .filter(|&&v| v)
        .count()
}

fn check_dirs(grid: &str, i: usize, c: char, ci: usize, dirs: &mut Directions, line_size: usize) {
    let ci = ci + 1;
    if dirs.N && grid.chars().nth(i - line_size * ci).unwrap_or(' ') != c {
        dirs.N = false;
    }
    if dirs.NE && grid.chars().nth(i - line_size * ci + ci).unwrap_or(' ') != c {
        dirs.NE = false;
    }
    if dirs.E && grid.chars().nth(i + ci).unwrap_or(' ') != c {
        dirs.E = false;
    }
    if dirs.SE && grid.chars().nth(i + line_size * ci + ci).unwrap_or(' ') != c {
        dirs.SE = false;
    }
    if dirs.S && grid.chars().nth(i + line_size * ci).unwrap_or(' ') != c {
        dirs.S = false;
    }
    if dirs.SW && grid.chars().nth(i + line_size * ci - ci).unwrap_or(' ') != c {
        dirs.SW = false;
    }
    if dirs.W && grid.chars().nth(i - ci).unwrap_or(' ') != c {
        dirs.W = false;
    }
    if dirs.NW && grid.chars().nth(i - line_size * ci - ci).unwrap_or(' ') != c {
        dirs.NW = false;
    }
}

fn part_two() {}