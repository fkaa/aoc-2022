use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .trim_end_matches('\n')
        .split('\n')
        .map(|r| {
            let (a, b) = r.as_bytes().split_at(r.len() / 2);
            let intersection = to_inventory_map(a) & to_inventory_map(b);

            intersection.trailing_zeros()
        })
        .sum()
}

fn to_inventory_map(chars: &[u8]) -> u64 {
    chars.iter().fold(0, |acc, c| acc | to_priority_bits(*c))
}

fn to_priority_bits(ch: u8) -> u64 {
    let priority = if ch >= 97 { ch - b'a' } else { ch - b'A' + 26 };

    1 << (1 + priority) as u64
}
