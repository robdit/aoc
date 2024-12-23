#![allow(clippy::needless_return)]

use std::{collections::VecDeque, vec};

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day15.txt");
    part1(&lines);
    part2(&lines);
}

fn hash(input: &[char]) -> usize {
    let mut cur = 0;
    for c in input {
        cur += *c as usize;
        cur *= 17;
        cur %= 256;
    }
    return cur;
}

fn part1(lines: &Vec<String>) {
    let inputs: Vec<Vec<char>> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();
    println!("{:?}", inputs);
    let mut tot = 0;
    for input in &inputs {
        tot += hash(input);
    }
    println!("part 1: {:?}", tot);
}

#[derive(PartialEq, Clone, Debug)]
struct Lens<'a> {
    id: &'a [char],
    focal: u32,
}

fn part2(lines: &Vec<String>) {
    let inputs: Vec<Vec<char>> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();
    let mut hash_map: Vec<VecDeque<Lens>> = vec![VecDeque::new(); 256];
    for input in &inputs {
        if *input.last().unwrap() == '-' {
            let mut to_remove = None;
            let id = &input[..input.len() - 1];
            let idx = hash(id);

            for (i, item) in hash_map[idx].iter().enumerate() {
                if item.id == id {
                    to_remove = Some(i);
                    break;
                }
            }
            if let Some(i) = to_remove {
                hash_map[idx].remove(i);
            }
        } else {
            let mut replaced = false;
            let idx = hash(&input[..input.len() - 2]);
            let id = &input[..input.len() - 2];
            for item in hash_map[idx].iter_mut() {
                if item.id == id {
                    item.focal = input[input.len() - 1].to_digit(10).unwrap();
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                hash_map[idx].push_back(Lens {
                    id: id.try_into().unwrap(),
                    focal: input[input.len() - 1].to_digit(10).unwrap(),
                });
            }
        }
    }
    let mut tot = 0;
    for (i, h) in hash_map.iter().enumerate() {
        if !h.is_empty() {
            for (j, item) in h.iter().enumerate() {
                tot += (i + 1) * (j + 1) * item.focal as usize;
            }
        }
    }
    println!("part2: {:?}", tot);
}
