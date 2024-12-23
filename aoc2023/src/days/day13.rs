#![allow(clippy::needless_return)]

use std::{collections::HashMap, result};

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day13.txt");
    part1(&lines);
    part2(&lines);
}

fn check_reflection(pattern: &Vec<Vec<char>>) -> usize {
    for i in 1..pattern.len() {
        let upper = (i + i).min(pattern.len());
        let lower = (i - (upper - i)).max(0);
        let p1: Vec<_> = pattern[lower..i].iter().collect();
        let p2: Vec<_> = pattern[i..upper].iter().rev().collect();
        let mut tot = 0;
        for i in 0..p1.len() {
            tot += (p1[i] == p2[i]) as usize;
        }
        if tot == p1.len() {
            return i * 100;
        }
    }

    for i in 1..pattern[0].len() {
        let mut tot = 0;
        for j in 0..pattern.len() {
            let upper = (i + i).min(pattern[0].len());
            let lower = (i - (upper - i)).max(0);
            tot += pattern[j][lower..i]
                .iter()
                .eq(pattern[j][i..upper].iter().rev()) as usize;
        }
        if tot == pattern.len() {
            return i;
        }
    }
    return 0;
}

fn check_reflection2(pattern: &Vec<Vec<char>>, avoid: usize) -> usize {
    for i in 1..pattern.len() {
        let upper = (i + i).min(pattern.len());
        let lower = (i - (upper - i)).max(0);
        let p1: Vec<_> = pattern[lower..i].iter().collect();
        let p2: Vec<_> = pattern[i..upper].iter().rev().collect();
        let mut tot = 0;
        for i in 0..p1.len() {
            tot += (p1[i] == p2[i]) as usize;
        }
        if tot == p1.len() {
            if avoid == i * 100 {
                continue;
            }
            return i * 100;
        }
    }

    for i in 1..pattern[0].len() {
        let mut tot = 0;
        for j in 0..pattern.len() {
            let upper = (i + i).min(pattern[0].len());
            let lower = (i - (upper - i)).max(0);
            tot += pattern[j][lower..i]
                .iter()
                .eq(pattern[j][i..upper].iter().rev()) as usize;
        }
        if tot == pattern.len() {
            if avoid == i {
                continue;
            }
            return i;
        }
    }
    return 0;
}

fn part1(lines: &Vec<String>) {
    let mut pattern: Vec<Vec<char>> = Vec::new();
    let mut tot = 0;
    for line in lines {
        if line.is_empty() {
            tot += check_reflection(&pattern);
            pattern.clear();
            continue;
        }
        pattern.push(line.chars().collect());
    }
    tot += check_reflection(&pattern);
    println!("{:?}", tot);
}

fn part2(lines: &Vec<String>) {
    let mut pattern: Vec<Vec<char>> = Vec::new();
    let mut tot = 0;
    for line in lines {
        if line.is_empty() {
            let avoid = check_reflection(&pattern);
            'free: for i in 0..pattern.len() {
                for j in 0..pattern[0].len() {
                    let mut np = pattern.clone();
                    np[i][j] = if np[i][j] == '#' { '.' } else { '#' };
                    let res = check_reflection2(&np, avoid);
                    if res != 0 {
                        tot += res;
                        break 'free;
                    }
                }
            }
            pattern.clear();
            continue;
        }
        pattern.push(line.chars().collect());
    }
    let avoid = check_reflection(&pattern);
    'free: for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            let mut np = pattern.clone();
            np[i][j] = if np[i][j] == '#' { '.' } else { '#' };
            let res = check_reflection2(&np, avoid);
            if res != 0 {
                tot += res;
                break 'free;
            }
        }
    }
    println!("{:?}", tot);
}
