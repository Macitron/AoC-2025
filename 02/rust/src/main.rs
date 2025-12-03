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

    let part2_answer = part2(&ranges);
    println!("Part 2: Invalid ID sum = {part2_answer}");
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

fn part2(ranges: &[(u64, u64)]) -> u64 {
    let mut invalid_id_sum = 0;
    /* Ideas
    start with the first char, look forward for that char, see if the string is a multiple of that
    distance between them. if it is, start checking the match in tandem
     */

    for (start, end) in ranges {
        for n in *start..=*end {
            let n_chars = n.to_string().chars().collect::<Vec<char>>();
            let first_char = n_chars.get(0).unwrap();

            for sublen in 1..=n_chars.len() / 2 {
                if n_chars.get(sublen).unwrap() == first_char
                    && n_chars.len() % sublen == 0
                    && check_substr(&n_chars, sublen)
                {
                    invalid_id_sum += n;
                    break;
                }
            }
        }
    }

    invalid_id_sum
}

// Invariants:
// 1. `sublen` divides str.len()
// 2. `str[sublen]` is equal to `str[0]`
// so we're just checking those substrings
fn check_substr(n_chars: &[char], sublen: usize) -> bool {
    let first_substr = &n_chars[0..sublen];
    let repeats = n_chars.len() / sublen;

    for iter in 1..repeats {
        let offset = iter * sublen;
        if &n_chars[offset..offset + sublen] != first_substr {
            return false;
        }
    }

    true
}
