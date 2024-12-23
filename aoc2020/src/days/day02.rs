#![allow(clippy::needless_return)]
#[allow(dead_code)]
pub fn solve() {
    let lines = common::read_file("./src/days/data/day02.txt");
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut tot = 0;

    for line in lines {
        let passwords: Vec<_> = line
            .replace(": ", ":")
            .split(':')
            .map(std::string::ToString::to_string)
            .collect();
        let policy: Vec<_> = passwords[0].split(' ').collect();
        let occurances: Vec<_> = policy[0]
            .split('-')
            .map(|x| x.parse::<usize>().expect("should be number"))
            .collect();
        let strength: Vec<_> = passwords[1]
            .chars()
            .filter(|x| x.to_string() == policy[1])
            .collect();
        if strength.len() >= occurances[0] && strength.len() <= occurances[1] {
            tot += 1;
        }
    }
    println!("part 1 total: {tot}");
}

fn part2(lines: &Vec<String>) {
    let mut tot2 = 0;

    for line in lines {
        let passwords: Vec<_> = line
            .replace(": ", ":")
            .split(':')
            .map(std::string::ToString::to_string)
            .collect();
        let policy: Vec<_> = passwords[0].split(' ').collect();
        let occurances: Vec<_> = policy[0]
            .split('-')
            .map(|x| x.parse::<usize>().expect("should be number"))
            .collect();
        let search_char = policy[1].chars().next().unwrap();
        if (passwords[1].chars().nth(occurances[0] - 1).unwrap() == search_char)
            != (passwords[1].chars().nth(occurances[1] - 1).unwrap() == search_char)
        {
            tot2 += 1;
        }
    }
    println!("part 2 total: {tot2}");
}
