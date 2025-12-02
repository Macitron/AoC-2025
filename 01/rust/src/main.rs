use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut infile = BufReader::new(File::open("../input").unwrap());
    let part1_answer = solve(&mut infile, &Part::One);
    println!("Part 1: password = {part1_answer}");

    infile = BufReader::new(File::open("../input").unwrap());
    let part2_answer = solve(&mut infile, &Part::Two);
    println!("Part 2: password = {part2_answer}");
}

enum Part {
    One,
    Two,
}

fn solve(reader: &mut impl BufRead, part: &Part) -> u32 {
    let mut zeros = 0;
    let mut pos = 50;

    for line in reader.lines() {
        let line = line.unwrap();
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
