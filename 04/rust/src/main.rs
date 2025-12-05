use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let infile = BufReader::new(File::open("../input").unwrap());
    let rolls_rows = infile
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let part1_ans = part1(&rolls_rows);
    println!("Part 1: {part1_ans} rolls can be accessed");
}

fn part1(rows: &[Vec<char>]) -> u32 {
    let mut crowded_rolls = 0;

    for (row_idx, row) in rows.iter().enumerate() {
        for col_idx in 0..row.len() {
            if is_crowded(&rows, row_idx, col_idx) {
                crowded_rolls += 1;
            }
        }
    }

    crowded_rolls
}

fn is_crowded(rows: &[Vec<char>], row_idx: usize, col_idx: usize) -> bool {
    let c = rows[row_idx][col_idx];
    if c != '@' {
        return false;
    }

    let mut rolls = 0;
    for vert_offset in -1..=1 {
        for horiz_offset in -1..=1 {
            if vert_offset == 0 && horiz_offset == 0 {
                continue;
            }

            if get_char_offset(&rows, (row_idx as isize, col_idx as isize), vert_offset, horiz_offset) == '@' {
                rolls += 1
            }
        }
    }

    rolls < 4
}

fn get_char_offset(
    rows: &[Vec<char>],
    base_pos: (isize, isize),
    vert_offset: isize,
    horiz_offset: isize,
) -> char {
    if base_pos.0 == 0 && vert_offset == -1 || base_pos.1 == 0 && horiz_offset == -1 {
        return '.';
    }

    rows.get((base_pos.0 + vert_offset) as usize).map_or_else(|| '.', |row| {
        *row.get((base_pos.1 + horiz_offset) as usize).unwrap_or_else(|| &'.')
    })
}
