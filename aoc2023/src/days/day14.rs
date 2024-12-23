#![allow(clippy::needless_return)]

use core::num;
use std::{collections::HashMap, result};

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day14_test.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in lines {
        data.push(line.chars().collect());
    }
    println!("{:?}", data);
    for i in 0..data.len() {
        for j in 1..data.len() - i {
            for c in 0..data[i].len() {
                if data[j - 1][c] == '.' && data[j][c] == 'O' {
                    data[j - 1][c] = 'O';
                    data[j][c] = '.';
                }
            }
        }
    }
    let mut tot = 0;
    for (i, d) in data.iter().enumerate() {
        tot += (data.len() - i) * d.iter().filter(|x| **x == 'O').count();
    }
    println!("part 1: {:?}", tot);
}

fn part2(lines: &Vec<String>) {
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in lines {
        data.push(line.chars().collect());
    }
    let num_cycles = 1_000_000_000;
    for cycle in 0..num_cycles {
        if cycle % 1000 == 0 {
            let mut tot = 0;
            for (i, d) in data.iter().enumerate() {
                tot += (data.len() - i) * d.iter().filter(|x| **x == 'O').count();
            }
            println!("cycle {:?} - {:?}", cycle, tot);
        }
        //north
        for i in 0..data.len() {
            for j in 1..data.len() - i {
                for c in 0..data[i].len() {
                    if data[j - 1][c] == '.' && data[j][c] == 'O' {
                        data[j - 1][c] = 'O';
                        data[j][c] = '.';
                    }
                }
            }
        }
        //west
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                for c in 1..data[i].len() - j {
                    if data[i][c - 1] == '.' && data[i][c] == 'O' {
                        data[i][c - 1] = 'O';
                        data[i][c] = '.';
                    }
                }
            }
        }
        //south
        for i in 0..data.len() {
            for j in (i..data.len() - 1).rev() {
                for c in 0..data[i].len() {
                    if data[j + 1][c] == '.' && data[j][c] == 'O' {
                        data[j + 1][c] = 'O';
                        data[j][c] = '.';
                    }
                }
            }
        }
        //east
        for i in 0..data.len() {
            for j in (0..data[i].len()).rev() {
                for c in j..data[i].len() - 1 {
                    if data[i][c + 1] == '.' && data[i][c] == 'O' {
                        data[i][c + 1] = 'O';
                        data[i][c] = '.';
                    }
                }
            }
        }
    }
    let mut tot = 0;
    for (i, d) in data.iter().enumerate() {
        tot += (data.len() - i) * d.iter().filter(|x| **x == 'O').count();
    }
    println!("part 2: {:?}", tot);
}
