#![allow(clippy::needless_return)]
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day03.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut trees = 0;
    let mut x = 0;
    for line in lines {
        if line.chars().nth(x).expect("to be there") == '#' {
            trees += 1;
        }
        x += 3;
        x %= line.len();
    }
    println!("part 1: {trees} hit");
}
fn part2(lines: &[String]) {
    let mut trees: [usize; 5] = [0; 5];
    let slopes = [1, 3, 5, 7, 1];
    let mut results: [usize; 5] = [0; 5];

    for (ln, line) in lines.iter().enumerate() {
        for (i, slope) in slopes.iter().enumerate() {
            if i == 4 && ln % 2 != 0 {
                continue;
            }
            if line.chars().nth(results[i]).expect("to be there") == '#' {
                trees[i] += 1;
            }
            results[i] += slope;
            results[i] %= line.len();
        }
    }
    println!("part 2: {:?} hit", trees.iter().product::<usize>());
}
