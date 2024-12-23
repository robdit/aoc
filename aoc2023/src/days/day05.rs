#![allow(clippy::needless_return)]

use std::{collections::HashMap, usize};
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day05.txt");
    part1(&lines);
    part2(&lines);
}

#[derive(Debug)]
struct Range {
    start: usize,
    stop: usize,
}

impl Range {
    fn new(start: usize, step: usize) -> Self {
        return Range {
            start,
            stop: start + step - 1,
        };
    }
    fn contains(&self, num: usize) -> bool {
        return num >= self.start && num <= self.stop;
    }
}

#[derive(Debug)]
struct SourceToDest {
    source: Range,
    dest: Range,
}

impl SourceToDest {
    fn new(source: Range, dest: Range) -> Self {
        return SourceToDest { source, dest };
    }

    fn dest(&self, source: usize) -> usize {
        let offset = source - self.source.start;
        return self.dest.start + offset;
    }
    fn source(&self, dest: usize) -> usize {
        let offset = dest - self.dest.start;
        return self.source.start + offset;
    }
}

fn part1(lines: &Vec<String>) {
    let mut source_dest_map: HashMap<String, Vec<SourceToDest>> = HashMap::new();
    let seeds: Vec<_> = lines[0]
        .replace("seeds: ", "")
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut i = 2;
    let mut range_type: String = String::new();
    let mut range_types: Vec<String> = Vec::new();
    while i < lines.len() {
        if lines[i].chars().next().unwrap().is_alphabetic() {
            range_type = lines[i].clone();
            range_types.push(range_type.clone());
            i += 1;
            continue;
        }
        let mut ranges: Vec<SourceToDest> = Vec::new();
        while i < lines.len() && !lines[i].is_empty() {
            let s2s: Vec<_> = lines[i]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            ranges.push(SourceToDest::new(
                Range::new(s2s[1], s2s[2]),
                Range::new(s2s[0], s2s[2]),
            ));
            i += 1;
        }
        source_dest_map.insert(range_type.clone(), ranges);
        i += 1;
    }
    let mut vals = seeds;
    for range in &range_types {
        for i in 0..vals.len() {
            for s2d in source_dest_map.get(range).unwrap() {
                if s2d.source.contains(vals[i]) {
                    vals[i] = s2d.dest(vals[i]);
                    break;
                }
            }
        }
    }
    println!("part 1: {:?}", vals.iter().min())
}

fn part2(lines: &Vec<String>) {
    let mut source_dest_map: HashMap<String, Vec<SourceToDest>> = HashMap::new();
    let seeds: Vec<_> = lines[0]
        .replace("seeds: ", "")
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut i = 2;
    let mut range_type: String = String::new();
    let mut range_types: Vec<String> = Vec::new();
    while i < lines.len() {
        if lines[i].chars().next().unwrap().is_alphabetic() {
            range_type = lines[i].clone();
            range_types.push(range_type.clone());
            i += 1;
            continue;
        }
        let mut ranges: Vec<SourceToDest> = Vec::new();
        while i < lines.len() && !lines[i].is_empty() {
            let s2s: Vec<_> = lines[i]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            ranges.push(SourceToDest::new(
                Range::new(s2s[1], s2s[2]),
                Range::new(s2s[0], s2s[2]),
            ));
            i += 1;
        }
        source_dest_map.insert(range_type.clone(), ranges);
        i += 1;
    }
    let mut seed_ranges: Vec<Range> = Vec::new();
    for seed in seeds.chunks(2) {
        seed_ranges.push(Range::new(seed[0], seed[1]))
    }

    let mut vals: Vec<usize> = Vec::new();
    for i in 0..50_000 {
        vals.push(i * 100_000);
    }
    for range in range_types.iter().rev() {
        for i in 0..vals.len() {
            for s2d in source_dest_map.get(range).unwrap() {
                if s2d.dest.contains(vals[i]) {
                    vals[i] = s2d.source(vals[i]);
                    break;
                }
            }
        }
    }
    let mut min_range: Option<&Range> = None;
    let mut min_val: Option<usize> = None;
    'free: for val in &vals {
        for range in &seed_ranges {
            if range.contains(*val) {
                println!("{:?}", range.start);
                println!("{val:?}");

                min_range = Some(range);
                min_val = Some(*val);
                break 'free;
            }
        }
    }
    let mut final_vals: Vec<usize> = Vec::new();

    match (min_range, min_val) {
        (Some(min), Some(end)) => {
            final_vals = (min.start..=end).collect();
            for range in &range_types {
                println!("====== {range:?} ======");
                for i in 0..final_vals.len() {
                    for s2d in source_dest_map.get(range).unwrap() {
                        if s2d.source.contains(final_vals[i]) {
                            final_vals[i] = s2d.dest(final_vals[i]);
                            break;
                        }
                    }
                }
            }
        }
        _ => (),
    }
    println!("{:?}", final_vals.iter().min())
}
