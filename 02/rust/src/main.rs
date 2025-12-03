use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = BufReader::new(File::open("../input").unwrap());

    let ranges = infile
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|range_str| {
            let (start_str, end_str) = range_str.split_once('-').unwrap();
            (
                start_str.parse::<u64>().unwrap(),
                end_str.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();

    let part1_answer = part1(&ranges);
    println!("Part 1: Invalid ID sum = {part1_answer}");
}

fn part1(ranges: &[(u64, u64)]) -> u64 {
    let mut kinda_palindrome_sum = 0;

    for (start, end) in ranges {
        for n in *start..=*end {
            if n.ilog10() % 2 == 0 {
                continue;
            }

            let n_str = n.to_string();
            let (first, second) = n_str.split_at(n_str.len() / 2);
            // lmao
            if first == second {
                kinda_palindrome_sum += n;
            }
        }
    }

    // what do you even call this
    kinda_palindrome_sum
}
