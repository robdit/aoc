#![allow(clippy::needless_return)]

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day10.txt");
    part1(&lines);
    part2(&lines);
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn complete(
    grid: &Vec<Vec<char>>,
    queu: &mut VecDeque<(isize, isize)>,
    oldpos: (isize, isize),
) -> usize {
    let mut pos = queu.front().unwrap().clone();
    let mut oold_pos = oldpos;
    while grid[pos.0 as usize][pos.1 as usize] != 'S' {
        let ch = grid[pos.0 as usize][pos.1 as usize];
        let pdiff = (oold_pos.0 - pos.0, oold_pos.1 - pos.1);
        let npos = match ch {
            '|' => [(1, 0), (-1, 0)],
            '-' => [(0, 1), (0, -1)],
            'L' => [(-1, 0), (0, 1)],
            'J' => [(-1, 0), (0, -1)],
            '7' => [(1, 0), (0, -1)],
            'F' => [(1, 0), (0, 1)],
            _ => unreachable!(),
        };
        let mut x = (-1, -1);
        for p in npos {
            if pdiff != p {
                queu.push_front((pos.0 + p.0, pos.1 + p.1));
                x = (pos.0 + p.0, pos.1 + p.1);
            }
        }
        oold_pos = pos;
        pos = queu.front().unwrap().clone();
    }
    return 0;
}

fn part1(lines: &Vec<String>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    let mut queu: VecDeque<(isize, isize)> = VecDeque::new();
    for (i, line) in lines.iter().enumerate() {
        grid.push(line.chars().collect());
        if let Some(idx) = line.find('S') {
            start = (i as isize, idx as isize);
            for dir in DIRECTIONS {
                if let Some(ch) = grid
                    .get((i as isize + dir.0) as usize)
                    .and_then(|x| x.get((idx as isize + dir.1) as usize))
                {
                    match (ch, dir) {
                        ('|', (1, 0) | (-1, 0))
                        | ('-', (0, -1) | (0, 1))
                        | ('L', (1, 0) | (0, -1))
                        | ('J', (1, 0) | (0, 1))
                        | ('7', (-1, 0) | (0, 1))
                        | ('F', (-1, 0) | (0, -1)) => {
                            queu.push_front((i as isize + dir.0, idx as isize + dir.1))
                        }
                        _ => (),
                    };
                }
            }
        }
    }
    let x = complete(&grid, &mut queu, start);
    println!("{:?}", queu.len() / 2);
}

fn part2(lines: &Vec<String>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    let mut queu: VecDeque<(isize, isize)> = VecDeque::new();
    for (i, line) in lines.iter().enumerate() {
        grid.push(line.chars().collect());
        if let Some(idx) = line.find('S') {
            start = (i as isize, idx as isize);
            for dir in DIRECTIONS {
                if let Some(ch) = grid
                    .get((i as isize + dir.0) as usize)
                    .and_then(|x| x.get((idx as isize + dir.1) as usize))
                {
                    match (ch, dir) {
                        ('|', (1, 0) | (-1, 0))
                        | ('-', (0, -1) | (0, 1))
                        | ('L', (1, 0) | (0, -1))
                        | ('J', (1, 0) | (0, 1))
                        | ('7', (-1, 0) | (0, 1))
                        | ('F', (-1, 0) | (0, -1)) => {
                            queu.push_front((i as isize + dir.0, idx as isize + dir.1))
                        }
                        _ => (),
                    };
                }
            }
        }
    }
    let x = complete(&grid, &mut queu, start);
    let mut ngrid = grid.clone();
    let mut nngrid = grid.clone();

    for p in &queu {
        ngrid[p.0 as usize][p.1 as usize] = 'X';
    }
    for i in 0..nngrid.len() {
        for j in 0..nngrid[0].len() {
            if ngrid[i][j] != 'X' {
                nngrid[i][j] = '.';
            }
        }
    }
    for n in &nngrid {
        println!("{:?}", n);
    }
    let mut tot = 0;
    for i in 0..nngrid.len() {
        for j in 0..nngrid[0].len() {
            if nngrid[i][j] != '.' {
                continue;
            }
            let mut col = j;
            let mut inloop = 0;

            let mut last = ' ';

            while col < nngrid[0].len() {
                let ch = nngrid[i][col];
                if ch == '|' || (last == 'L' && ch == '7') || (last == 'F' && ch == 'J') {
                    inloop += 1;
                }
                // s in big input is a - cba coding it out
                if ch != '-' && ch != 'S' {
                    last = ch;
                }

                col += 1;
            }
            if inloop % 2 == 1 {
                tot += 1;
            }
        }
    }
    println!("total: {tot:?}");
}
