#![allow(clippy::needless_return)]

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day03.txt");
    part1(&lines);
    part2(&lines);
}

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn part1(lines: &Vec<String>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    let mut num = String::new();
    let mut valid = false;
    let mut tot: usize = 0;
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if ch.to_digit(10).is_none() {
                if valid {
                    tot += num.parse::<usize>().expect("to be a number");
                    valid = false;
                }
                num.clear();
                continue;
            }
            num.push(*ch);
            if valid {
                continue;
            }
            for dir in DIRECTIONS {
                let x = i as isize + dir.0;
                if x < 0 || x >= grid.len() as isize {
                    continue;
                }
                let y = j as isize + dir.1;
                if y < 0 || y >= grid[i].len() as isize {
                    continue;
                }
                if grid[x as usize][y as usize] != '.'
                    && grid[x as usize][y as usize].to_digit(10).is_none()
                {
                    valid = true;
                    break;
                }
            }
        }
        if valid {
            tot += num.parse::<usize>().expect("to be a number");
            valid = false;
        }
        num.clear();
    }

    println!("{tot}");
}

fn part2(lines: &Vec<String>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    let mut num = String::new();
    let mut valid = false;
    let mut tot: usize = 0;
    let width = grid[0].len();
    let mut gear_power: Vec<usize> = vec![0; width * grid.len()];
    let mut gear_count: Vec<usize> = vec![0; width * grid.len()];
    let mut gear_pos: (usize, usize) = (0, 0);
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_ascii_digit() {
                if valid {
                    let number = num.parse::<usize>().expect("to be a number");
                    gear_count[gear_pos.0 * width + gear_pos.1] += 1;
                    if gear_power[gear_pos.0 * width + gear_pos.1] == 0 {
                        gear_power[gear_pos.0 * width + gear_pos.1] = number;
                    } else {
                        gear_power[gear_pos.0 * width + gear_pos.1] *= number;
                    }
                    valid = false;
                }
                gear_pos = (0, 0);
                num.clear();
                continue;
            }
            num.push(*ch);
            if valid {
                continue;
            }
            for dir in DIRECTIONS {
                let x = i as isize + dir.0;
                if x < 0 || x >= grid.len() as isize {
                    continue;
                }
                let y = j as isize + dir.1;
                if y < 0 || y >= grid[i].len() as isize {
                    continue;
                }
                if grid[x as usize][y as usize] == '*' {
                    valid = true;
                    gear_pos = (x as usize, y as usize);
                    break;
                }
            }
        }
        if valid {
            let number = num.parse::<usize>().expect("to be a number");
            gear_count[gear_pos.0 * width + gear_pos.1] += 1;
            if gear_power[gear_pos.0 * width + gear_pos.1] == 0 {
                gear_power[gear_pos.0 * width + gear_pos.1] = number;
            } else {
                gear_power[gear_pos.0 * width + gear_pos.1] *= number;
            }
            valid = false;
        }
        num.clear();
    }
    for (i, count) in gear_count.iter().enumerate() {
        if *count == 2 {
            tot += gear_power[i];
        }
    }

    println!("{tot}");
}
