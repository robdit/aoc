#![allow(clippy::needless_return)]

#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day09.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut sequences: Vec<Vec<isize>> = Vec::new();
    for line in lines {
        sequences.push(
            line.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    let mut tot = 0;
    for seq in sequences.iter_mut() {
        let mut deriv_seqs: Vec<Vec<isize>> = Vec::new();
        deriv_seqs.push(seq.windows(2).map(|x| x[1] - x[0]).collect::<Vec<isize>>());
        while !deriv_seqs.is_empty() && deriv_seqs.last().unwrap().iter().sum::<isize>() != 0 {
            deriv_seqs.push(
                deriv_seqs
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<isize>>(),
            )
        }
        for i in (0..deriv_seqs.len() - 1).rev() {
            let new_val = deriv_seqs[i + 1].last().unwrap() + deriv_seqs[i].last().unwrap();
            deriv_seqs[i].push(new_val);
        }
        let last_seq = *deriv_seqs[0].last().unwrap() + seq.last().unwrap();
        seq.push(last_seq);
        tot += last_seq;
    }
    println!("part 1: {:?}", tot);
}

fn part2(lines: &Vec<String>) {
    let mut sequences: Vec<Vec<isize>> = Vec::new();
    for line in lines {
        sequences.push(
            line.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .rev()
                .collect::<Vec<_>>(),
        );
    }
    let mut tot = 0;
    for seq in sequences.iter_mut() {
        let mut deriv_seqs: Vec<Vec<isize>> = Vec::new();
        deriv_seqs.push(seq.windows(2).map(|x| x[1] - x[0]).collect::<Vec<isize>>());
        while !deriv_seqs.is_empty() && deriv_seqs.last().unwrap().iter().sum::<isize>() != 0 {
            deriv_seqs.push(
                deriv_seqs
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<isize>>(),
            )
        }
        for i in (0..deriv_seqs.len() - 1).rev() {
            let new_val = deriv_seqs[i + 1].last().unwrap() + deriv_seqs[i].last().unwrap();
            deriv_seqs[i].push(new_val);
        }
        let last_seq = *deriv_seqs[0].last().unwrap() + seq.last().unwrap();
        seq.push(last_seq);
        tot += last_seq;
    }
    println!("part 2: {:?}", tot);
}
