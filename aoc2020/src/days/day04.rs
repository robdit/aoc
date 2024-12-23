#![allow(clippy::needless_return)]

use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day04.txt");
    part1(&lines);
    part2(&lines);
}

fn from_fields(fields: &Vec<String>) -> HashMap<String, String> {
    let mut passport: HashMap<String, String> = HashMap::new();
    for field in fields {
        let parts: Vec<_> = field.split(':').collect();
        passport.insert(parts[0].to_string(), parts[1].to_string());
    }
    return passport;
}

fn validate_passport(np: &Vec<String>) -> bool {
    let passport = from_fields(np);
    return (passport.keys().len() == 7 && !passport.contains_key("cid"))
        || passport.keys().len() == 8;
}

fn validate_passport_p2(np: &Vec<String>) -> bool {
    if !validate_passport(np) {
        return false;
    }
    let passport = from_fields(np);
    let color_re = Regex::new(r"^#([a-f0-9]{6})$").unwrap();
    let pid_re = Regex::new(r"^([0-9]{9})$").unwrap();
    for (k, v) in &passport {
        let val = match k.as_str() {
            "byr" => {
                let year = v.parse::<usize>().unwrap();
                (1920..=2002).contains(&year)
            }
            "iyr" => {
                let year = v.parse::<usize>().unwrap();
                (2010..=2020).contains(&year)
            }
            "eyr" => {
                let year = v.parse::<usize>().unwrap();
                (2020..=2030).contains(&year)
            }
            "hgt" => {
                if v.contains("cm") {
                    let height = v.replace("cm", "").parse::<usize>().unwrap();
                    (150..=193).contains(&height)
                } else if v.contains("in") {
                    let height = v.replace("in", "").parse::<usize>().unwrap();
                    (59..=76).contains(&height)
                } else {
                    false
                }
            }
            "hcl" => color_re.is_match(v),
            "ecl" => matches!(
                v.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            ),
            "pid" => pid_re.is_match(v),
            "cid" => true,
            _ => unreachable!(),
        };
        if !val {
            return false;
        }
    }
    return true;
}

fn part1(lines: &Vec<String>) {
    let mut np: Vec<String> = Vec::new();
    let mut tot = 0;
    for line in lines {
        if line.is_empty() {
            if validate_passport(&np) {
                tot += 1;
            }
            np.clear();
            continue;
        }
        let fields: Vec<_> = line
            .split(' ')
            .map(std::string::ToString::to_string)
            .collect();
        np.extend(fields);
    }
    if validate_passport(&np) {
        tot += 1;
    }
    println!("{tot:?}");
}
fn part2(lines: &Vec<String>) {
    let mut np: Vec<String> = Vec::new();
    let mut tot = 0;
    for line in lines {
        if line.is_empty() {
            if validate_passport_p2(&np) {
                tot += 1;
            }
            np.clear();
            continue;
        }
        let fields: Vec<_> = line
            .split(' ')
            .map(std::string::ToString::to_string)
            .collect();
        np.extend(fields);
    }
    if validate_passport_p2(&np) {
        tot += 1;
    }
    println!("{tot:?}");
}
