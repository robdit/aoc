#![allow(clippy::needless_return)]

use std::{collections::HashMap, result};

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day12.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut total = 0;
    for line in lines {
        let split: Vec<_> = line.split_whitespace().collect();
        let lava: Vec<char> = split[0].chars().collect();
        let groups: Vec<_> = split[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut combinations = vec![vec![]];
        for _ in 0..lava.iter().filter(|&x| *x == '?').count() {
            let mut new_combinations = Vec::new();
            for combination in &combinations {
                let mut new_comb = combination.clone();
                new_comb.push('#');
                new_combinations.push(new_comb);
                let mut new_comb = combination.clone();
                new_comb.push('.');
                new_combinations.push(new_comb);
            }
            combinations = new_combinations;
        }
        let mut tot = 0;
        for c in &combinations {
            let mut nc = lava.clone();
            let mut j = 0;
            for i in 0..lava.len() {
                if nc[i] == '?' {
                    nc[i] = c[j];
                    j += 1;
                }
            }
            let mut group_idxs: Vec<usize> = vec![];
            let mut gc = 0;
            for c in &nc {
                if *c == '.' && gc > 0 {
                    group_idxs.push(gc);
                    gc = 0;
                } else if *c == '#' {
                    gc += 1;
                }
            }
            if gc > 0 {
                group_idxs.push(gc);
            }
            if group_idxs == groups {
                tot += 1;
            }
        }

        total += tot;
        //println!("{:?} == {:?} == {:?}", group_idxs, groups, combs);
    }
    println!("{:?}", total);
}

fn recursive(
    lava: &[char],
    groups: &[usize],
    cache: &mut HashMap<(*const char, *const usize), usize>,
) -> usize {
    if groups.is_empty() {
        return (!lava.contains(&'#')) as usize;
    }
    let current = groups[0];
    let next_group = &groups[1..];
    let mut i = 0;
    let mut res = 0;
    while i < lava.len() - next_group.iter().sum::<usize>() - next_group.len() - current + 1 {
        if lava.get(..i).unwrap_or(&[]).contains(&'#') {
            break;
        }
        let next = i + current;
        if next <= lava.len()
            && !lava.get(i..next).unwrap_or(&[]).contains(&'.')
            && lava.get(next..next + 1) != Some(&['#'])
        {
            let lv = lava.get(next + 1..).unwrap_or(&[]);
            if lv.is_empty() && next_group.is_empty() {
                res += recursive(&lava.get(next + 1..).unwrap_or(&[]), next_group, cache);
            } else {
                let k = (lv.as_ptr(), next_group.as_ptr());
                if let Some(v) = cache.get(&k) {
                    res += v;
                } else {
                    let val = recursive(&lava.get(next + 1..).unwrap_or(&[]), next_group, cache);
                    cache.insert(k, val);
                    res += val;
                }
            }
        }
        i += 1;
    }
    return res;
}

fn part2(lines: &Vec<String>) {
    let mut total = 0;
    for line in lines {
        let split: Vec<_> = line.split_whitespace().collect();
        let lava: Vec<char> = split[0].chars().collect();
        let groups: Vec<_> = split[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut new_lava = lava.clone();
        let mut new_groups = groups.clone();
        for _ in 0..4 {
            new_lava.push('?');
            new_lava.extend(lava.iter().cloned());
            new_groups.extend(groups.iter().cloned());
        }
        let lava = new_lava;
        let groups = new_groups;
        let res = recursive(&lava, &groups, &mut HashMap::new());
        println!("{:?}", res);
        total += res;
    }
    println!("{:?}", total);
}
