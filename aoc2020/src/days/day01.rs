#![allow(clippy::needless_return)]
#[allow(dead_code)]
pub fn solve() {
    let mut inputs: [bool; 2020] = [false; 2020];
    let lines = common::read_file("./src/days/data/day01.txt");
    for line in &lines {
        let index: usize = line.parse().expect("to be a number");
        inputs[index] = true;
        if inputs[2020 - index] {
            println!("part 1: {:?}", index * (2020 - index));
            break;
        }
    }
    'free: for line in &lines {
        let index: usize = line.parse().expect("to be a number");
        inputs[index] = true;
        for i in (0..(inputs.len() - index)).rev() {
            if inputs[i] && inputs[2020 - index - i] {
                println!("part 2: {:?}", index * i * (2020 - index - i));
                break 'free;
            }
        }
    }
}
