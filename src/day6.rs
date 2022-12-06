use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> usize {
    input
        .array_windows::<4>()
        .position(|&w| !has_dupes(w))
        .unwrap_or(0)
        + 4
}

fn has_dupes([a, b, c, d]: [u8; 4]) -> bool {
    a == b || a == c || a == d || b == c || b == d || c == d
}

#[aoc(day6, part2)]
pub fn part2(input: &[u8]) -> usize {
    let mut state = [0u8; 26];
    for c in &input[..14] {
        let b = c - b'a';
        state[b as usize] += 1;
    }

    input
        .array_windows::<15>()
        .scan(state, |state, c| {
            if state.iter().all(|&c| c <= 1) {
                return None;
            }

            let old = c[0] - b'a';
            let new = c[14] - b'a';
            state[old as usize] -= 1;
            state[new as usize] += 1;

            Some(())
        })
        .count()
        + 14
}
