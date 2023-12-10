fn parse_part1(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn startswith(s: &str, prefix: &str) -> bool {
    s.len() >= prefix.len() && &s[..prefix.len()] == prefix
}

fn parse_part2(line: &str) -> Vec<u32> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    line.chars()
        .enumerate()
        .filter_map(|(i, c)| match c.to_digit(10) {
            Some(d) => Some(d),
            None => digits
                .iter()
                .enumerate()
                .filter_map(|(j, &d)| {
                    if startswith(&line[i..], d) {
                        Some((j + 1) as u32)
                    } else {
                        None
                    }
                })
                .take(1)
                .collect::<Vec<_>>()
                .first()
                .copied(),
        })
        .collect()
}

fn run(input: &[String], parse_fn: fn(&str) -> Vec<u32>) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            let nums = parse_fn(line);
            Some(nums.first()? * 10 + nums.last()?)
        })
        .sum::<_>()
}

pub fn part1(input: &[String]) -> u32 {
    run(input, parse_part1)
}

pub fn part2(input: &[String]) -> u32 {
    run(input, parse_part2)
}
