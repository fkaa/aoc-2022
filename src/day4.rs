use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .trim_end_matches('\n')
        .split('\n')
        .filter_map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let ((a_from, a_to), (b_from, b_to)) = (parse_range(a), parse_range(b));

            if (a_from >= b_from && a_to <= b_to) || (b_from >= a_from && b_to <= a_to) {
                Some(())
            } else {
                None
            }
        })
        .count() as u32
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .trim_end_matches('\n')
        .split('\n')
        .filter_map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let ((a_from, a_to), (b_from, b_to)) = (parse_range(a), parse_range(b));

            if a_from <= b_to && b_from <= a_to {
                Some(())
            } else {
                None
            }
        })
        .count() as u32
}

fn parse_range(val: &str) -> (u32, u32) {
    let (from, to) = val.split_once('-').unwrap();

    (from.parse().unwrap(), to.parse().unwrap())
}
