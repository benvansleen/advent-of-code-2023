use runner::run_puzzle;
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

    let output = run_puzzle!(read_input, day, part);
    println!("{:#?}", output);
}


#[cfg(test)]
mod tests {
    #[test]
    fn regression_day1() {
        use advent_of_code_2023::day1;
        let input = super::read_input("1");
        assert_eq!(day1::part1(&input), 55834);
        assert_eq!(day1::part2(&input), 53221);
    }

    #[test]
    fn regression_day2() {
        use advent_of_code_2023::day2;
        let input = super::read_input("2");
        assert_eq!(day2::part1(&input), 2683);
        assert_eq!(day2::part2(&input), 49710);
    }

    #[test]
    fn regression_day3() {
        use advent_of_code_2023::day3;
        let input = super::read_input("3");
        assert_eq!(day3::part1(&input), 527369);
        assert_eq!(day3::part2(&input), 73074886);
    }

    #[test]
    fn regression_day4() {
        use advent_of_code_2023::day4;
        let input = super::read_input("4");
        assert_eq!(day4::part1(&input), 19855);
        assert_eq!(day4::part2(&input), 10378710);
    }

    #[test]
    fn regression_day5() {
        use advent_of_code_2023::day5;
        let input = super::read_input("5");
        assert_eq!(day5::part1(&input), 389056265);
        // Expensive test (~3 min):
        // assert_eq!(day5::part2(&input), 137516820);
    }
}
