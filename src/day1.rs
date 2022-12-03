use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|c| c.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut elf_calories = input
        .split("\n\n")
        .map(|c| c.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    elf_calories.sort_unstable();
    elf_calories.iter().rev().take(3).sum()
}
