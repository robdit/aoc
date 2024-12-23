#![allow(clippy::needless_return)]

use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day08.txt");
    part1(&lines);
    part2(&lines);
}

#[derive(Debug)]
struct Node {
    val: [char; 3],
    left: [char; 3],
    right: [char; 3],
}

impl Node {
    fn path(&self, ch: char) -> [char; 3] {
        if ch == 'L' {
            return self.left;
        }
        return self.right;
    }
}

fn part1(lines: &Vec<String>) {
    let instructions: Vec<char> = lines[0].chars().collect();
    let mut nodes: HashMap<[char; 3], Node> = HashMap::new();
    for line in lines.iter().skip(2) {
        let caps = Regex::new("([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)")
            .unwrap()
            .captures(line)
            .unwrap();
        nodes.insert(
            caps[1].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
            Node {
                val: caps[1].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
                left: caps[2].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
                right: caps[3].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
            },
        );
    }
    let mut idx = 0;
    let mut steps = 0;
    let mut curr: [char; 3] = ['A', 'A', 'A'];
    while curr != ['Z', 'Z', 'Z'] {
        curr = nodes[&curr].path(instructions[idx]);
        idx = (idx + 1) % instructions.len();
        steps += 1;
    }
    println!("part 1: {steps}");
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

fn part2(lines: &Vec<String>) {
    let instructions: Vec<char> = lines[0].chars().collect();
    let mut nodes: HashMap<[char; 3], Node> = HashMap::new();
    for line in lines.iter().skip(2) {
        let caps = Regex::new("([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)")
            .unwrap()
            .captures(line)
            .unwrap();
        nodes.insert(
            caps[1].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
            Node {
                val: caps[1].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
                left: caps[2].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
                right: caps[3].chars().collect::<Vec<_>>()[..3].try_into().unwrap(),
            },
        );
    }
    let mut idx = 0;
    let mut steps = 0;
    let mut curr: Vec<_> = nodes.keys().filter(|x| x[2] == 'A').cloned().collect();
    let mut stops: Vec<usize> = vec![0; curr.len()];
    for i in 0..curr.len() {
        while curr[i][2] != 'Z' {
            curr[i] = nodes[&curr[i]].path(instructions[idx]);
            idx = (idx + 1) % instructions.len();
            steps += 1;
        }
        stops[i] = steps;
        steps = 0;
        idx = 0;
    }
    println!("part 2: {:?}", stops.iter().fold(1, |a, &b| lcm(a, b)));
}
