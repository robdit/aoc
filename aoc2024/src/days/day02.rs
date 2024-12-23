#![allow(clippy::needless_return)]
#[allow(dead_code)]

fn safe_check(levels: &[isize]) -> bool {
    let mut dir = 0;
    for win in levels.windows(2) {
        let dif: isize = win[0] - win[1];
        if dif == 0 || dif.abs() > 3 {
            return false;
        }
        if dif < 0 {
            if dir == 0 {
                dir = -1;
            } else if dir > 0 {
                return false;
            }
        } else if dif > 0 {
            if dir == 0 {
                dir = 1;
            } else if dir < 0 {
                return false;
            }
        }
    }
    return true;
}
pub fn solve() {
    let lines = common::read_file("./src/days/data/day02.txt");
    let mut tot = 0;
    for line in &lines {
        let levels: Vec<isize> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        if safe_check(&levels) {
            tot += 1;
        }
    }
    println!("day 02 part 1: {tot}");
    let mut tot2 = 0;
    for line in &lines {
        let levels: Vec<isize> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        let safe = safe_check(&levels);
        if safe {
            tot2 += 1;
        } else {
            for i in 0..levels.len() {
                let mut dampened = levels.clone();
                dampened.remove(i);
                if safe_check(&dampened) {
                    tot2 += 1;
                    break;
                }
            }
        }
    }
    println!("day 02 part 2: {tot2}");
}
