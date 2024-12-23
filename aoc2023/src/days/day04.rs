#![allow(clippy::needless_return)]
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day04.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut tot = 0;
    for line in lines {
        let mut draws: [usize; 99] = [0; 99];
        let p1: Vec<_> = line
            .split(':')
            .map(std::string::ToString::to_string)
            .collect();
        let numbers: Vec<_> = p1[1]
            .split('|')
            .map(std::string::ToString::to_string)
            .collect();

        numbers[1]
            .split(' ')
            .filter(|x| !x.is_empty())
            .for_each(|x| {
                draws[x.parse::<usize>().unwrap() - 1] += 1;
            });
        let draw = numbers[0]
            .split(' ')
            .map(std::string::ToString::to_string)
            .filter(|x| !x.is_empty())
            .fold(0, |acc, x| {
                if draws[x.parse::<usize>().unwrap() - 1] != 0 {
                    if acc == 0 {
                        return 1;
                    } else {
                        return acc * 2;
                    }
                }
                acc
            });
        tot += draw;
    }
    println!("part 1: {tot:?}");
}

fn part2(lines: &Vec<String>) {
    let mut tot = 0;
    let mut card = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let mut draws: [usize; 99] = [0; 99];
        let p1: Vec<_> = line
            .split(':')
            .map(std::string::ToString::to_string)
            .collect();
        let numbers: Vec<_> = p1[1]
            .split('|')
            .map(std::string::ToString::to_string)
            .collect();

        numbers[1]
            .split(' ')
            .filter(|x| !x.is_empty())
            .for_each(|x| {
                draws[x.parse::<usize>().unwrap() - 1] += 1;
            });
        let draw = numbers[0]
            .split(' ')
            .map(std::string::ToString::to_string)
            .filter(|x| !x.is_empty())
            .fold(0, |acc, x| {
                if draws[x.parse::<usize>().unwrap() - 1] != 0 {
                    return acc + 1;
                }
                acc
            });
        println!("draw: {draw:?} - {i:?}");
        for j in i..i + draw {
            println!("{j:?}");
            card[j + 1] += card[i];
        }
        tot += draw;
    }
    println!("anw: {:?}", card.iter().sum::<usize>());

    println!("{tot:?}");
}
