use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = BufReader::new(File::open("../input").unwrap());
    let lines = infile
        .lines()
        .map(|res| res.unwrap())
        .collect::<Vec<String>>();

    let part1_answer = solve(&lines, &Part::One);
    println!("Part 1: password = {part1_answer}");

    let part2_answer = solve(&lines, &Part::Two);
    println!("Part 2: password = {part2_answer}");
}

enum Part {
    One,
    Two,
}

fn solve(lines: &[String], part: &Part) -> u32 {
    let mut zeros = 0;
    let mut pos = 50;

    for line in lines {
        let (dir, count) = line.split_at(1);
        let add = count.parse::<i32>().unwrap() * if dir == "L" { -1 } else { 1 };

        zeros += match part {
            Part::One => {
                let new_pos = (pos + add).rem_euclid(100);
                u32::from(new_pos == 0)
            }
            Part::Two => {
                let raw_sum = pos + add;
                match raw_sum.cmp(&0) {
                    Ordering::Greater => (raw_sum / 100).unsigned_abs(),
                    Ordering::Equal => 1,
                    Ordering::Less => u32::from(pos != 0) + (raw_sum / 100).unsigned_abs(),
                }
            }
        };
        pos = (pos + add).rem_euclid(100);
    }

    zeros
}
