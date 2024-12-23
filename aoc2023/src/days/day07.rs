#![allow(clippy::needless_return)]
use std::convert::TryInto;

use std::{cmp::Ordering, collections::HashMap};
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day07.txt");
    part1(&lines);
    part2(&lines);
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [usize; 5],
    strength: Kind,
    bet: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength == other.strength {
            for i in 0..self.cards.len() {
                if self.cards[i] < other.cards[i] {
                    return Ordering::Less;
                } else if self.cards[i] > other.cards[i] {
                    return Ordering::Greater;
                }
            }
        } else if self.strength < other.strength {
            return Ordering::Less;
        } else if self.strength > other.strength {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HandP2 {
    cards: [usize; 5],
    strength: Kind,
    bet: usize,
}

impl Ord for HandP2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.strength == other.strength {
            for i in 0..self.cards.len() {
                if self.cards[i] < other.cards[i] {
                    return Ordering::Less;
                } else if self.cards[i] > other.cards[i] {
                    return Ordering::Greater;
                }
            }
        } else if self.strength < other.strength {
            return Ordering::Less;
        } else if self.strength > other.strength {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for HandP2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Kind {
    NoValue,
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Hand {
    fn new(hand: &str, bet: usize) -> Self {
        let cards = hand
            .chars()
            .map(|x| match x {
                '2'..='9' => x.to_digit(10).unwrap() as usize,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()[..5]
            .try_into()
            .unwrap();
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in hand.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let keys: Vec<_> = map.keys().collect();
        let strength = match map.keys().len() {
            1 => Kind::FiveKind,
            2 => {
                if *map.get(keys[0]).unwrap() == 1 || *map.get(keys[0]).unwrap() == 4 {
                    Kind::FourKind
                } else {
                    Kind::FullHouse
                }
            }
            3 => {
                let mut is_three_kind = false;
                let mut strength: Kind = Kind::TwoPair;
                for k in &keys {
                    if *map.get(k).unwrap() == 3 {
                        is_three_kind = true;
                    }
                }
                if is_three_kind {
                    strength = Kind::ThreeKind
                }
                strength
            }
            4 => {
                let mut strength: Kind = Kind::NoValue;
                for k in &keys {
                    if *map.get(k).unwrap() == 2 {
                        strength = Kind::OnePair;
                    }
                }
                strength
            }
            5 => Kind::HighCard,
            _ => Kind::NoValue,
        };
        return Self {
            cards,
            strength,
            bet,
        };
    }
}

impl HandP2 {
    fn new(hand: &str, bet: usize) -> Self {
        let mut cards: [usize; 5] = hand
            .chars()
            .map(|x| match x {
                '2'..='9' => x.to_digit(10).unwrap() as usize,
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()[..5]
            .try_into()
            .unwrap();
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in hand.chars() {
            if c == 'J' {
                continue;
            }
            *map.entry(c).or_insert(0) += 1;
        }

        let _keys: Vec<_> = map.keys().collect();
        let mut max_type: usize = 1;
        let mut max_count: usize = 0;
        println!("{hand:?}");

        for (k, v) in &map {
            let val = match k {
                '2'..='9' => k.to_digit(10).unwrap() as usize,
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            };
            if (*v > max_count) || (*v == max_count && val > max_type) {
                max_type = val;
                max_count = *v;
            }
            println!("{:?}-{:?}-{:?}-{:?}", *v, val, max_count, max_type);
        }
        let orig_cards = cards;
        for i in 0..cards.len() {
            if cards[i] == 1 {
                cards[i] = max_type;
            }
        }
        let mut map: HashMap<usize, usize> = HashMap::new();
        for c in &cards {
            *map.entry(*c).or_insert(0) += 1;
        }
        let keys: Vec<_> = map.keys().collect();
        let strength = match map.keys().len() {
            1 => Kind::FiveKind,
            2 => {
                if *map.get(&cards[0]).unwrap() == 1 || *map.get(&cards[0]).unwrap() == 4 {
                    Kind::FourKind
                } else {
                    Kind::FullHouse
                }
            }
            3 => {
                let mut is_three_kind = false;
                let mut strength: Kind = Kind::TwoPair;
                for k in &keys {
                    if *map.get(k).unwrap() == 3 {
                        is_three_kind = true;
                    }
                }
                if is_three_kind {
                    strength = Kind::ThreeKind
                }
                strength
            }
            4 => {
                let mut strength: Kind = Kind::NoValue;
                for k in &keys {
                    if *map.get(k).unwrap() == 2 {
                        strength = Kind::OnePair;
                    }
                }
                strength
            }
            5 => Kind::HighCard,
            _ => Kind::NoValue,
        };
        return Self {
            cards: orig_cards,
            strength,
            bet,
        };
    }
}

fn part1(lines: &Vec<String>) {
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        hands.push(Hand::new(parts[0], parts[1].parse::<usize>().unwrap()));
    }
    hands.sort();
    let mut tot = 0;
    for (i, hand) in hands.iter().enumerate() {
        tot += hand.bet * (i + 1);
    }
    println!("part 1: {tot:?}");
}

fn part2(lines: &Vec<String>) {
    let mut hands: Vec<HandP2> = Vec::new();
    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        hands.push(HandP2::new(parts[0], parts[1].parse::<usize>().unwrap()));
    }
    hands.sort();
    let mut tot = 0;
    for (i, hand) in hands.iter().enumerate() {
        println!("{hand:?}");
        tot += hand.bet * (i + 1);
    }
    // println!("{:?}", hands);
    println!("{tot:?}");
}
