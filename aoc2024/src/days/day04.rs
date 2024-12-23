#![allow(clippy::needless_return)]
#[allow(dead_code)]

fn word_search(
    needle: &[char],
    haystack: &[Vec<char>],
    coord: (isize, isize),
    dir: (isize, isize),
    curr: &mut Vec<char>,
) -> bool {
    if curr == needle {
        return true;
    }
    if curr.len() >= needle.len() {
        return false;
    }
    if coord.0 < 0 || coord.0 >= haystack.len() as isize {
        return false;
    }
    if coord.1 < 0 || coord.1 >= haystack[coord.0 as usize].len() as isize {
        return false;
    }
    curr.push(haystack[coord.0 as usize][coord.1 as usize]);
    return word_search(
        needle,
        haystack,
        (coord.0 + dir.0, coord.1 + dir.1),
        dir,
        curr,
    );
}

pub fn solve() {
    let lines = common::read_file("./src/days/data/day04.txt");
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        grid.push(line.chars().collect());
    }
    let dirs = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut tot = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for dir in dirs.iter() {
                if word_search(
                    &['X', 'M', 'A', 'S'],
                    &grid,
                    (i as isize, j as isize),
                    *dir,
                    &mut Vec::new(),
                ) {
                    tot += 1;
                }
            }
        }
    }

    println!("day 04 part 1: {:?}", tot);

    let mut tot2 = 0;
    for row in 1..grid.len() - 1 {
        for col in 1..grid[row].len() - 1 {
            if grid[row][col] == 'A' {
                if ((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
                    || (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M'))
                    && ((grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
                        || (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M'))
                {
                    tot2 += 1;
                }
            }
        }
    }
    println!("day 04 part 2: {:?}", tot2);
}
