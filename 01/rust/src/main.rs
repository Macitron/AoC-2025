use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut infile = BufReader::new(File::open("../input").unwrap());

    let part1_answer = part1(&mut infile);
    println!("Part 1: password = {part1_answer}");
}

fn part1(reader: &mut impl BufRead) -> u32 {
    let mut zeros = 0;
    let mut sum = 50;

    for line in reader.lines() {
        let line = line.unwrap();
        let (dir, count) = line.split_at(1);
        let operand = count.parse::<i32>().unwrap() * if dir == "L" { -1 } else { 1 };

        sum = (sum + operand + 100) % 100;
        if sum == 0 {
            zeros += 1;
        }
    }

    zeros
}
