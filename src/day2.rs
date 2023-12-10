use std::collections::HashMap;

#[derive(Debug)]
struct State {
    n_blue: u32,
    n_red: u32,
    n_green: u32,
}

impl State {
    fn from(s: &str) -> State {
        let mut color_count = HashMap::from([("blue", 0), ("red", 0), ("green", 0)]);

        s.split(',').map(|i| i.trim()).for_each(|split| {
            let split_on_space: Vec<&str> = split.split(' ').collect();
            let count: u32 = split_on_space
                .first()
                .expect("No count found")
                .parse::<u32>()
                .expect("Count is not a number");
            let color: &str = split_on_space.last().expect("No color found");
            color_count.insert(color, count);
        });

        State {
            n_blue: *color_count.get("blue").unwrap_or(&0),
            n_red: *color_count.get("red").unwrap_or(&0),
            n_green: *color_count.get("green").unwrap_or(&0),
        }
    }

    fn power(&self) -> u32 {
        self.n_blue * self.n_red * self.n_green
    }
}

struct Game {
    id: u32,
    iterations: Vec<State>,
}

impl Game {
    fn from(s: &str) -> Game {
        let split_on_colon: Vec<&str> = s.split(':').collect();
        let game_id = split_on_colon
            .first()
            .expect("Incorrect input format -- no colon")
            .split(' ')
            .last()
            .expect("Incorrect input format -- no space")
            .parse::<u32>()
            .expect("Incorrect input format -- not a number");

        let iterations: Vec<State> = split_on_colon
            .last()
            .expect("Incorrect input format -- no colon")
            .split(';')
            .map(State::from)
            .collect();

        Game {
            id: game_id,
            iterations,
        }
    }

    fn is_valid(&self, constraint: &State) -> bool {
        self.iterations.iter().all(|i| {
            i.n_blue <= constraint.n_blue
                && i.n_red <= constraint.n_red
                && i.n_green <= constraint.n_green
        })
    }

    fn min_valid_set(&self) -> State {
        self.iterations.iter().fold(
            State {
                n_blue: u32::min_value(),
                n_red: u32::min_value(),
                n_green: u32::min_value(),
            },
            |acc, i| State {
                n_blue: std::cmp::max(acc.n_blue, i.n_blue),
                n_red: std::cmp::max(acc.n_red, i.n_red),
                n_green: std::cmp::max(acc.n_green, i.n_green),
            },
        )
    }
}

pub fn part1(input: &[String]) -> u32 {
    let constraint = State {
        n_blue: 14,
        n_red: 12,
        n_green: 13,
    };

    input
        .iter()
        .map(|line| Game::from(line))
        .filter(|game| game.is_valid(&constraint))
        .fold(0, |acc, game| acc + game.id)
}

pub fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| Game::from(line).min_valid_set())
        .fold(0, |acc, i| acc + i.power())
}
