use crate::common::BoundedInt;
use std::collections::HashMap;

#[derive(Debug)]
struct State<T> {
    n_blue: T,
    n_red: T,
    n_green: T,
}

impl<T: std::fmt::Display> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "blue: {}, red: {}, green: {}",
            self.n_blue, self.n_red, self.n_green
        )
    }
}

impl<T: BoundedInt> Default for State<T> {
    fn default() -> Self {
        Self {
            n_blue: T::min_value(),
            n_red: T::min_value(),
            n_green: T::min_value(),
        }
    }
}

impl<T: BoundedInt> State<T>
{
    fn from(s: &str) -> Self {
        let mut color_count: HashMap<&str, T> = HashMap::from([
            ("blue", T::min_value()),
            ("red", T::min_value()),
            ("green", T::min_value()),
        ]);

        s.split(',').map(|i| i.trim()).for_each(|split| {
            let split_on_space: Vec<&str> = split.split(' ').collect();
            let count: T = split_on_space
                .first()
                .expect("No count found")
                .parse()
                .unwrap_or(T::min_value());
            let color: &str = split_on_space.last().expect("No color found");
            color_count.insert(color, count);
        });

        let get_count = |color: &str| -> T {
            *color_count.get(color).unwrap_or(&T::min_value())
        };

        Self {
            n_blue: get_count("blue"),
            n_red: get_count("red"),
            n_green: get_count("green"),
        }
    }

    fn power(&self) -> T {
        self.n_blue * self.n_red * self.n_green
    }
}

#[derive(Debug)]
struct Game<T> {
    id: T,
    iterations: Vec<State<T>>,
}

impl<T: std::fmt::Display> std::fmt::Display for Game<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game {}", self.id)?;
        for (i, state) in self.iterations.iter().enumerate() {
            writeln!(f, "Iteration {i}: {state}")?;
        }
        Ok(())
    }
}

impl<T: BoundedInt> Game<T>
{
    fn from(s: &str) -> Game<T> {
        let split_on_colon: Vec<&str> = s.split(':').collect();
        let game_id: T = split_on_colon
            .first()
            .expect("Incorrect input format -- no colon")
            .split(' ')
            .last()
            .expect("Incorrect input format -- no space")
            .parse()
            .unwrap_or(T::min_value());

        let iterations: Vec<State<_>> = split_on_colon
            .last()
            .expect("Incorrect input format -- no colon")
            .split(';')
            .map(State::<_>::from)
            .collect();

        Self {
            id: game_id,
            iterations,
        }
    }

    fn is_valid(&self, constraint: &State<T>) -> bool {
        self.iterations.iter().all(|i| {
            i.n_blue <= constraint.n_blue
                && i.n_red <= constraint.n_red
                && i.n_green <= constraint.n_green
        })
    }

    fn min_valid_set(&self) -> State<T> {
        self.iterations
            .iter()
            .fold(State::<_>::default(), |acc, i| State {
                n_blue: std::cmp::max(acc.n_blue, i.n_blue),
                n_red: std::cmp::max(acc.n_red, i.n_red),
                n_green: std::cmp::max(acc.n_green, i.n_green),
            })
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
        .inspect(|line| log::debug!("Parsing line:\n{line}"))
        .map(|line| Game::<u16>::from(line))
        .inspect(|game| log::debug!("Parsed game:\n{game}"))
        .filter(|game| game.is_valid(&constraint))
        .fold(0, |acc, game| acc + game.id) as u32
}

pub fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .inspect(|line| log::debug!("Parsing line:\n{line}"))
        .map(|line| Game::<u16>::from(line).min_valid_set())
        .inspect(|state| log::debug!("Min viable state:\n{state}"))
        .fold(0, |acc, i| acc + i.power()) as u32
}
