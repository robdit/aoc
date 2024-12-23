#![allow(clippy::needless_return)]

use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day03.txt");
    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    let mut tot = 0;
    for line in &lines {
        for cap in re.captures_iter(line) {
            let muls = &cap[1]
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            tot += muls[0] * muls[1];
        }
    }
    println!("day 03 part 1: {:?}", tot);
    let mut tot2 = 0;
    let mut enabled = true;
    for line in &lines {
        let chars = line.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == 'd' {
                if &line[i..i + 4] == "do()" {
                    enabled = true;
                    i += 4;
                } else if &line[i..i + 7] == "don't()" {
                    enabled = false;
                    i += 7;
                }
            } else if chars[i] == 'm' && &line[i..i + 4] == "mul(" && enabled {
                i += 4;
                let mut e = i;
                while chars[e] >= '0' && chars[e] <= '9' {
                    e += 1;
                }
                let f = &line[i..e].parse::<isize>().unwrap();
                i = e;
                if chars[i] != ',' {
                    continue;
                }
                i += 1;
                e = i;
                while chars[e] >= '0' && chars[e] <= '9' {
                    e += 1;
                }
                let s = &line[i..e].parse::<isize>().unwrap();
                i = e;
                if chars[i] != ')' {
                    continue;
                }
                i += 1;
                tot2 += f * s;
            } else {
                i += 1;
            }
        }
    }
    println!("day 03 part 2: {:?}", tot2);
}
