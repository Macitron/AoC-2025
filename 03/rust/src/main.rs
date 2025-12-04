use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = BufReader::new(File::open("../input").unwrap());
    let banks = infile
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let part1_answer: u64 = banks.iter().map(|bank| max_joltage_p1(bank)).sum();
    println!("Part 1: Max Joltage = {part1_answer}");

    let part2_answer: u64 = banks.iter().map(|bank| max_joltage_p2(bank)).sum();
    println!("Part 2: Max Joltage = {part2_answer}");
}

fn max_joltage_p1(battery: &[u64]) -> u64 {
    let (first_pos, max_first) = find_max(&battery[0..battery.len() - 1]);
    if first_pos == battery.len() - 2 {
        return max_first * 10 + battery.last().unwrap();
    }

    let (_, max_second) = find_max(&battery[first_pos + 1..battery.len()]);
    max_first * 10 + max_second
}

fn max_joltage_p2(battery: &[u64]) -> u64 {
    let mut maxes: [(usize, u64); 12] = [(0, 0); 12];

    for i in 0..12 {
        let start_pos = if i == 0 { 0 } else { maxes[i - 1].0 + 1 };
        let end_pos = battery.len() - (12 - i - 1);
        let (slice_pos, max_i) = find_max(&battery[start_pos..end_pos]);
        let max_pos = start_pos + slice_pos;

        maxes[i] = (max_pos, max_i);
        if max_pos == battery.len() - (12 - i) {
            copy_rest(&battery[max_pos..], &mut maxes[i..]);
            break;
        }
    }

    let mut sum = 0;
    for (pos, (_, x)) in maxes.iter().enumerate() {
        sum += x * 10u64.pow(11 - pos as u32);
    }

    sum
}

fn copy_rest(battery_slice: &[u64], maxes_slice: &mut [(usize, u64)]) {
    for (pos, &x) in battery_slice.iter().enumerate() {
        maxes_slice[pos] = (0, x);
    }
}

fn find_max(arr: &[u64]) -> (usize, u64) {
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
