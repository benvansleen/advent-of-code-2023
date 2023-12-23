use std::env;

fn fetch_input(day: &str) -> String {
    println!("Fetching input for day {}...", day);
    if !std::path::Path::new(".session").exists() {
        panic!("No session file found");
    }

    let session = std::fs::read_to_string(".session")
        .expect("could not read session file");
    let url = format!("https://adventofcode.com/2023/day/{day}/input",);

    ureq::get(&url)
        .set("Cookie", session.trim())
        .call()
        .expect("could not fetch input")
        .into_string()
        .expect("could not parse input")
}

fn read_input(day: &str) -> Vec<String> {
    if atty::is(atty::Stream::Stdin) {
        if !std::path::Path::new("inputs").exists() {
            std::fs::create_dir("inputs")
                .expect("Failed to create inputs directory");
        }

        let filename = format!("inputs/day{day}");

        let contents = match std::fs::read_to_string(&filename) {
            Ok(contents) => contents,
            Err(_) => {
                let contents = fetch_input(day);
                std::fs::write(&filename, &contents)
                    .expect("Failed to write input file");
                contents
            }
        };

        contents
            .trim()
            .split('\n')
            .map(|line| line.to_string())
            .collect()
    } else {
        std::io::stdin().lines().map_while(Result::ok).collect()
    }
}

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("missing day argument");
    let part = args.get(2).expect("missing part argument");
    let input = read_input(day);

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
