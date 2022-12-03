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


#[aoc(day1, part2, Faster)]
pub fn part2_opt(input: &str) -> u32 {
    let elf_calories = input
        .split("\n\n")
        .map(|c| c.lines().map(|l| l.parse::<u32>().unwrap()).sum());

    let mut top = [0, 0, 0];

    for calories in elf_calories {
        if calories > top[0] {
            top = [calories, top[0], top[1]];
        } else if calories > top[1] {
            top[2] = top[1];
            top[1] = calories;
        } else if calories > top[2] {
            top[2] = calories;
        }
    }

    top.iter().sum()
}
