#![allow(clippy::needless_return)]

use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day06.txt");
    part1(&lines);
    part2(&lines);
}

fn calc_wins(time: usize, target: usize) -> usize {
    let p1 = ((time.pow(2) - (4 * target)) as f64).sqrt();
    let pos = (time as f64 + p1) / 2.0;
    let neg = (time as f64 - p1) / 2.0;

    return (pos.floor() - neg.ceil()) as usize + 1;
}

fn part1(lines: &Vec<String>) {
    let times: Vec<usize> = Regex::new("Time: +")
        .unwrap()
        .replace_all(&lines[0], "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let distances: Vec<usize> = Regex::new("Distance: +")
        .unwrap()
        .replace_all(&lines[1], "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut tot: usize = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        tot *= calc_wins(*time, distance + 1);
    }
    println!("Part 1: {tot:?}");
}

fn part2(lines: &Vec<String>) {
    let time: usize = Regex::new("Time: +")
        .unwrap()
        .replace_all(&lines[0], "")
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    let distance: usize = Regex::new("Distance: +")
        .unwrap()
        .replace_all(&lines[1], "")
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    println!("time: {time:?}");
    println!("distance: {distance:?}");

    println!("{:?}", calc_wins(time, distance + 1));
}
