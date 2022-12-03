use aoc_runner_derive::aoc;

#[aoc(day2, part1, Safe)]
pub fn part1(input: &[u8]) -> u32 {
    input
        .array_chunks::<4>()
        .map(|[opponent, _, me, _]| LUT_PART1[(*opponent - b'A') as usize][(*me - b'X') as usize])
        .sum::<u32>()
}

#[aoc(day2, part1, Unsafe)]
pub fn part1_unsafe(input: &[u8]) -> u32 {
    input
        .array_chunks::<4>()
        .map(|[opponent, _, me, _]| unsafe {
            LUT_PART1
                .get_unchecked((*opponent - b'A') as usize)
                .get_unchecked((*me - b'X') as usize)
        })
        .sum::<u32>()
}

#[aoc(day2, part2, Safe)]
pub fn part2(input: &[u8]) -> u32 {
    input
        .array_chunks::<4>()
        .map(|[opponent, _, me, _]| LUT_PART2[(*opponent - b'A') as usize][(*me - b'X') as usize])
        .sum::<u32>()
}

#[aoc(day2, part2, Unsafe)]
pub fn part2_unsafe(input: &[u8]) -> u32 {
    input
        .array_chunks::<4>()
        .map(|[opponent, _, me, _]| unsafe {
            LUT_PART2
                .get_unchecked((*opponent - b'A') as usize)
                .get_unchecked((*me - b'X') as usize)
        })
        .sum::<u32>()
}

const LUT_PART1: [[u32; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
const LUT_PART2: [[u32; 3]; 3] = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
