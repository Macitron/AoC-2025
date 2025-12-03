use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = BufReader::new(File::open("../input").unwrap());
    let turns = infile
        .lines()
        .map(|res| res.unwrap())
        .map(|line| {
            let (dir, count) = line.split_at(1);
            count.parse::<i32>().unwrap() * if dir == "L" { -1 } else { 1 }
        })
        .collect::<Vec<i32>>();

    let part1_answer = solve(&turns, &Part::One);
    println!("Part 1: password = {part1_answer}");

    let part2_answer = solve(&turns, &Part::Two);
    println!("Part 2: password = {part2_answer}");
}

enum Part {
    One,
    Two,
}

fn solve(turns: &[i32], part: &Part) -> u32 {
    let mut zeros = 0;
    let mut pos = 50;

    for &turn in turns {
        zeros += match part {
            Part::One => {
                let new_pos = (pos + turn).rem_euclid(100);
                u32::from(new_pos == 0)
            }
            Part::Two => {
                let raw_sum = pos + turn;
                match raw_sum.cmp(&0) {
                    Ordering::Greater => (raw_sum / 100).unsigned_abs(),
                    Ordering::Equal => 1,
                    Ordering::Less => u32::from(pos != 0) + (raw_sum / 100).unsigned_abs(),
                }
            }
        };
        pos = (pos + turn).rem_euclid(100);
    }

    zeros
}
