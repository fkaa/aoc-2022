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

#[aoc(day4, part1, Faster)]
pub fn part1_opt(input: &str) -> u32 {
    input
        .trim_end_matches('\n')
        .split('\n')
        .filter(|l| {
            l.split_once(',')
                .map(|(a, b)| {
                    let ((a_from, a_to), (b_from, b_to)) =
                        (parse_range_fast(a), parse_range_fast(b));

                    (a_from >= b_from && a_to <= b_to) || (b_from >= a_from && b_to <= a_to)
                })
                .unwrap_or(false)
        })
        .count() as u32
}

#[aoc(day4, part2, Faster)]
pub fn part2_opt(input: &str) -> u32 {
    input
        .trim_end_matches('\n')
        .split('\n')
        .filter(|l| {
            l.split_once(',')
                .map(|(a, b)| {
                    let ((a_from, a_to), (b_from, b_to)) =
                        (parse_range_fast(a), parse_range_fast(b));

                    a_from <= b_to && b_from <= a_to
                })
                .unwrap_or(false)
        })
        .count() as u32
}

fn parse_range_fast(val: &str) -> (u32, u32) {
    if let Some((from, to)) = val.split_once('-') {
        (
            parse_num_fast(from.as_bytes()),
            parse_num_fast(to.as_bytes()),
        )
    } else {
        (0, 0)
    }
}

fn parse_num_fast(val: &[u8]) -> u32 {
    match *val {
        [a] => char_to_num(a),
        [a, b] => 10 * char_to_num(a) + char_to_num(b),
        _ => 0,
    }
}

fn char_to_num(ch: u8) -> u32 {
    (ch - b'0') as u32
}
