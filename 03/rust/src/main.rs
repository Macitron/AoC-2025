use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = BufReader::new(File::open("../input").unwrap());
    let banks = infile
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let part1_answer = part1(&banks);
    println!("Part 1: Max Joltage = {part1_answer}");
}

fn part1(banks: &[Vec<u32>]) -> u32 {
    banks.iter().map(|bank| max_joltage(bank)).sum()
}

fn max_joltage(battery: &[u32]) -> u32 {
    let (first_pos, max_first) = find_max(&battery[0..battery.len() - 1]);
    if first_pos == battery.len() - 2 {
        return max_first * 10 + battery.last().unwrap();
    }

    let (_, max_second) = find_max(&battery[first_pos + 1..battery.len()]);
    max_first * 10 + max_second
}

fn find_max(arr: &[u32]) -> (usize, u32) {
    let (max_pos, max) = arr
        .iter()
        .enumerate()
        .reduce(
            |(max_pos, max), (p, x)| {
                if x > max { (p, &x) } else { (max_pos, &max) }
            },
        )
        .unwrap();

    (max_pos, *max)
}
