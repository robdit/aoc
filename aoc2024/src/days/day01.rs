#![allow(clippy::needless_return)]

use std::{collections::HashMap, iter::zip};
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day01.txt");
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    let mut tot = 0;
    for line in &lines {
        let parts = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        left.push(parts[0]);
        right.push(parts[1]);
    }
    left.sort();
    right.sort();

    let mut tot2 = 0;
    let mut occ: HashMap<usize, usize> = HashMap::new();

    for r in &right {
        if let Some(v) = occ.get(r) {
            occ.insert(*r, v + 1);
        } else {
            occ.insert(*r, 1);
        }
    }
    for l in &left {
        if let Some(v) = occ.get(l) {
            tot2 += v * l;
        }
    }

    for (l, r) in zip(left, right) {
        tot += l.abs_diff(r);
    }
    println!("day 01 part 1: {tot}");
    println!("day 01 part 2: {tot2}");
}
