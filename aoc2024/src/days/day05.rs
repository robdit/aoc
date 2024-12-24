#![allow(clippy::needless_return)]

use std::collections::HashMap;

fn check_updates(
    updates: &[usize],
    rule_map: &HashMap<usize, Vec<usize>>,
    end: usize,
) -> (bool, usize) {
    for i in (0..end).rev() {
        for j in (0..i).rev() {
            if let Some(rules) = rule_map.get(&updates[i]) {
                if rules.contains(&updates[j]) {
                    return (false, i);
                }
            }
        }
    }
    return (true, 0);
}

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day05.txt");

    let mut rule_map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut rule_mode = true;
    let mut tot = 0;
    let mut tot2 = 0;
    for line in lines.iter() {
        if line.trim() == "" {
            rule_mode = false;
            continue;
        }
        if rule_mode {
            let order = line
                .split("|")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            rule_map
                .entry(order[0])
                .or_insert_with(Vec::new)
                .push(order[1]);
        } else {
            let updates = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let is_valid = check_updates(&updates, &rule_map, updates.len());
            if is_valid.0 {
                tot += updates[updates.len() / 2];
            } else {
                let mut new_updates: Vec<_> = Vec::with_capacity(updates.len());
                for i in 0..updates.len() {
                    for j in 0..=i {
                        let mut nn = new_updates.clone();
                        nn.insert(j, updates[i]);
                        if check_updates(&nn, &rule_map, i + 1).0 {
                            new_updates = nn;
                            break;
                        }
                    }
                }
                tot2 += new_updates[new_updates.len() / 2];
            }
        }
    }

    println!("day 05 part 1: {:?}", tot);
    println!("day 05 part 2: {:?}", tot2);
}
