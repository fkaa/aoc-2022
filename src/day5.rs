use aoc_runner_derive::aoc;

use crate::util;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = stacks(0);
    initialize_stacks(&mut stacks, crates.as_bytes());

    perform_moves(&mut stacks, moves);

    stacks
        .into_iter()
        .filter_map(|s| s.last().map(|b| *b as char))
        .collect::<String>()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = stacks(0);
    initialize_stacks(&mut stacks, crates.as_bytes());

    perform_moves_part2(&mut stacks, moves);

    stacks
        .into_iter()
        .filter_map(|s| s.last().map(|b| *b as char))
        .collect::<String>()
}

fn initialize_stacks(stacks: &mut [Vec<u8>; 9], crates: &[u8]) {
    for line in crates.iter().array_chunks::<36>().rev() {
        for i in 0..9 {
            let krate = *line[i * 4 + 1];

            if let b'A'..=b'Z' = krate {
                stacks[i].push(krate)
            }
        }
    }
}

fn perform_moves(stacks: &mut [Vec<u8>; 9], moves: &str) {
    for line in moves.lines() {
        let mut words = line.as_bytes().split(|b| *b == b' ');
        words.next();
        let amount = util::parse_num_fast(words.next().unwrap()) as usize;
        words.next();
        let from = util::parse_num_fast(words.next().unwrap()) as usize;
        words.next();
        let to = util::parse_num_fast(words.next().unwrap()) as usize;

        move_part1(stacks, from, to, amount);
    }
}

fn move_part1(stacks: &mut [Vec<u8>; 9], from: usize, to: usize, amount: usize) {
    let from = from - 1;
    let to = to - 1;
    let new_length = stacks[from].len() - amount;

    for i in 0..amount {
        stacks[to].push(stacks[from][stacks[from].len() - 1 - i]);
    }
    stacks[from].truncate(new_length);
}

fn perform_moves_part2(stacks: &mut [Vec<u8>; 9], moves: &str) {
    for line in moves.lines() {
        let mut words = line.as_bytes().split(|b| *b == b' ');
        words.next();
        let amount = util::parse_num_fast(words.next().unwrap()) as usize;
        words.next();
        let from = util::parse_num_fast(words.next().unwrap()) as usize;
        words.next();
        let to = util::parse_num_fast(words.next().unwrap()) as usize;

        move_part2(stacks, from, to, amount);
    }
}

fn move_part2(stacks: &mut [Vec<u8>; 9], from: usize, to: usize, amount: usize) {
    let (from, to) = if from > to {
        let (lo, hi) = stacks.split_at_mut(to);
        let to_in_lo = if to > 0 { to - 1 } else { to };
        let from_in_hi = if from - to > 0 { from - to - 1 } else { 0 };

        (&mut hi[from_in_hi], &mut lo[to_in_lo])
    } else {
        let (lo, hi) = stacks.split_at_mut(from);
        let from_in_lo = if from > 0 { from - 1 } else { from };
        let to_in_hi = if to - from > 0 { to - from - 1 } else { 0 };

        (&mut lo[from_in_lo], &mut hi[to_in_hi])
    };
    let new_length = from.len() - amount;

    to.extend_from_slice(&from[new_length..]);
    from.truncate(new_length);
}

fn stacks(capacity: usize) -> [Vec<u8>; 9] {
    [
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
        Vec::with_capacity(capacity),
    ]
}
