#![allow(clippy::needless_return)]

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day11.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut stars: Vec<(usize, usize)> = Vec::new();
    let mut offset = 0;
    let mut vspace = Vec::new();
    for i in 0..lines[0].len() {
        let mut found = false;
        for line in lines {
            if line.chars().nth(i).unwrap() != '.' {
                found = true;
            }
        }
        if !found {
            vspace.push(i);
        }
    }
    for (i, line) in lines.iter().enumerate() {
        let mut found = false;
        for (j, c) in line.char_indices() {
            let row_offset = vspace.iter().filter(|x| *x < &j).count();
            if c == '#' {
                stars.push((i + offset, j + row_offset));
                found = true;
            }
        }
        if !found {
            offset += 1;
        }
    }
    let mut pairs = Vec::new();
    let mut tot = 0;
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            pairs.push((stars[i], stars[j]));
            tot += stars[j].0.abs_diff(stars[i].0) + stars[j].1.abs_diff(stars[i].1);
        }
    }
    println!("part 1:{:?}", tot);
}

fn part2(lines: &Vec<String>) {
    let mut stars: Vec<(usize, usize)> = Vec::new();
    let mut offset = 0;
    let mut vspace = Vec::new();
    for i in 0..lines[0].len() {
        let mut found = false;
        for line in lines {
            if line.chars().nth(i).unwrap() != '.' {
                found = true;
            }
        }
        if !found {
            vspace.push(i);
        }
    }
    for (i, line) in lines.iter().enumerate() {
        let mut found = false;
        for (j, c) in line.char_indices() {
            let row_offset = vspace.iter().filter(|x| *x < &j).count() * 999_999;
            if c == '#' {
                stars.push((i + offset, j + row_offset));
                found = true;
            }
        }
        if !found {
            offset += 999_999;
        }
    }
    let mut pairs = Vec::new();
    let mut tot = 0;
    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            pairs.push((stars[i], stars[j]));
            tot += stars[j].0.abs_diff(stars[i].0) + stars[j].1.abs_diff(stars[i].1);
        }
    }
    println!("part 2: {:?}", tot);
}
