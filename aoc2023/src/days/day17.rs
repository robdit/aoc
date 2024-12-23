#![allow(clippy::needless_return)]
use std::{
    collections::{HashMap, VecDeque},
    thread::{self, Builder},
};
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day17.txt");
    part1(&lines);
    //part1(&lines);
    part2(&lines);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    No,
}

impl Directions {
    fn moves(&self) -> [Directions; 2] {
        return match self {
            Directions::Up => [Self::Left, Self::Right],
            Directions::Down => [Self::Left, Self::Right],
            Directions::Left => [Self::Up, Self::Down],
            Directions::Right => [Self::Up, Self::Down],
            Directions::No => [Self::Down, Self::Right],
        };
    }

    fn to_tuple(&self) -> (isize, isize) {
        return match self {
            Directions::Up => (1, 0),
            Directions::Down => (-1, 0),
            Directions::Left => (0, -1),
            Directions::Right => (0, 1),
            Directions::No => (0, 0),
        };
    }
}

fn bfs(map: &Vec<Vec<usize>>, min_moves: usize, max_moves: usize) -> Option<usize> {
    let n = map.len() as isize;
    let m = map[0].len() as isize;
    let mut que: VecDeque<((isize, isize), Directions, usize)> = VecDeque::new();
    let mut visted: HashMap<((isize, isize), Directions), usize> = HashMap::new();
    que.push_back((Directions::Right.to_tuple(), Directions::Right, 0));
    que.push_back((Directions::Up.to_tuple(), Directions::Up, 0));
    let mut min: Option<usize> = None;

    while let Some((coords, dir, loss)) = que.pop_front() {
        let prev_loss = visted.entry((coords, dir)).or_insert(loss + 1);
        let min_moves = if min_moves > 0 {
            min_moves - 1
        } else {
            min_moves
        };
        if *prev_loss > loss {
            *prev_loss = loss;
        } else {
            continue;
        }
        let mut loss = loss;
        let mut coords = coords;
        for i in 0..max_moves {
            if coords.0 < 0 || coords.0 >= n {
                break;
            }
            if coords.1 < 0 || coords.1 >= m {
                break;
            }
            loss += map[coords.0 as usize][coords.1 as usize];
            if coords.0 == n - 1 && coords.1 == m - 1 {
                if (min.is_none() || min.unwrap() > loss) && i >= min_moves {
                    min = Some(loss);
                }
                break;
            }

            if i >= min_moves {
                let moves = dir.moves();
                for m in moves {
                    let mdir = m.to_tuple();
                    que.push_back(((coords.0 + mdir.0, coords.1 + mdir.1), m, loss));
                }
            }
            let ndir = dir.to_tuple();
            coords = (coords.0 + ndir.0, coords.1 + ndir.1);
        }
    }
    return min;
}

fn part1(lines: &Vec<String>) {
    let mut data: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        data.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    let r = bfs(&data, 0, 3);
    println!("part 1: {:?}", r);
}

fn part2(lines: &Vec<String>) {
    let mut data: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        data.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    let r = bfs(&data, 4, 10);
    println!("part 2: {:?}", r);
}
