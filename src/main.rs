use std::env;

fn read_input(day: &str, part: &str) -> Vec<String> {
    if atty::is(atty::Stream::Stdin) {
        let filename = format!("inputs/day{day}/part{part}");
        std::fs::read_to_string(filename)
            .expect("could not read input file")
            .lines()
            .map(|line| line.to_string())
            .collect()
    } else {
        std::io::stdin()
            .lines()
            .map_while(Result::ok)
            .collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("missing day argument");
    let part = args.get(2).expect("missing part argument");
    let input = read_input(day, part);

    let output = match day.as_str() {
        "1" => {
            use advent_of_code_2023::day1::{part1, part2};
            match part.as_str() {
                "1" => part1(&input),
                "2" => part2(&input),
                _ => panic!("invalid part"),
            }
        }
        "2" => {
            use advent_of_code_2023::day2::{part1, part2};
            match part.as_str() {
                "1" => part1(&input),
                "2" => part2(&input),
                _ => panic!("invalid part"),
            }
        }
        _ => panic!("unknown day"),
    };

    println!("{output}");
}
