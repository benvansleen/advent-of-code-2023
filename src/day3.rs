use crate::common::BoundedInt;
use std::collections::HashMap;
use std::slice::Iter;

#[derive(Debug)]
struct Node<T> {
    symbol: char,
    children: Vec<T>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    row_i: usize,
    col_i: usize,
}

struct Grid<T> {
    nodes: HashMap<Point, Node<T>>,
}

#[derive(Debug)]
enum Token<T> {
    Number(T),
    Symbol(char),
}

fn read_next_token<T: BoundedInt>(s: &mut Iter<char>) -> (Token<T>, usize) {
    let parsed_num: Result<T, _> = str::parse(
        &s.clone()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>(),
    );
    match parsed_num {
        Ok(n) => {
            let n_digits = n.checked_log10().unwrap_or(0) + 1;
            (0..n_digits).for_each(|_| {
                s.next();
            });
            (Token::Number(n), n_digits)
        }
        Err(_) => (Token::Symbol(*s.next().unwrap()), 1),
    }
}

impl<T: BoundedInt> Grid<T> {
    fn from(input: &[String]) -> Self {
        let input: Vec<Vec<char>> =
            input.iter().map(|s| s.chars().collect()).collect();

        let (tx, rx) = std::sync::mpsc::channel();
        let producer = std::thread::spawn(move || {
            input.iter().enumerate().for_each(|(row_i, row)| {
                let mut col_i = 0;
                let mut s = row.iter();
                while col_i < row.len() {
                    let (token, n_digits) = read_next_token(&mut s);
                    log::debug!(
                        "Next token: {:?} ({:?} digits)",
                        token,
                        n_digits
                    );
                    if let Token::Number(n) = token {
                        if let Some((symbol, pt)) = Self::neighboring_symbol(
                            &input,
                            row_i,
                            col_i,
                            col_i + n_digits,
                        ) {
                            log::debug!(
                                "Found symbol {:?} at {:?}",
                                symbol,
                                pt
                            );
                            tx.send((pt, symbol, n)).unwrap();
                        }
                    }

                    col_i += n_digits;
                }
            })
        });

        let mut nodes: HashMap<Point, Node<T>> = HashMap::new();
        rx.iter().for_each(|(pt, symbol, n)| {
            nodes
                .entry(pt)
                .or_insert(Node {
                    symbol,
                    children: Vec::new(),
                })
                .children
                .push(n);
        });
        producer.join().unwrap();

        Self { nodes }
    }

    fn neighboring_symbol(
        input: &[Vec<char>],
        row_i: usize,
        col_i: usize,
        col_end: usize,
    ) -> Option<(char, Point)> {
        let symbol = |c: &char| {
            let c: char = *c;
            match c {
                '*' | '+' | '&' | '=' | '$' | '@' | '/' | '-' | '%' | '#' => {
                    Some(c)
                }
                _ => None,
            }
        };

        let check = |row_i: usize, col_i: usize| {
            symbol(input.get(row_i)?.get(col_i)?)
                .map(|c| (c, Point { row_i, col_i }))
        };

        check(row_i, col_i.saturating_sub(1))
            .or(check(row_i, col_end))
            .or((col_i.saturating_sub(1)..=col_end).fold(None, |acc, i| {
                acc.or(check(row_i.saturating_sub(1), i))
                    .or(check(row_i.saturating_add(1), i))
            }))
    }

    fn sum_non_orphans(&self) -> T {
        self.nodes
            .values()
            .inspect(|n| log::debug!("{}: {:?}", n.symbol, n.children))
            .map(|n| n.children.iter().fold(T::default(), |acc, n| acc + *n))
            .sum()
    }

    fn sum_and_multiply_non_orphans(&self) -> T {
        self.nodes
            .values()
            .filter(|node| node.symbol == '*' && node.children.len() == 2)
            .inspect(|node| log::debug!("{}: {:?}", node.symbol, node.children))
            .map(|node| node.children.clone().into_iter().product())
            .sum()
    }
}

pub fn part1(input: &[String]) -> u32 {
    let grid = Grid::<u32>::from(input);
    grid.sum_non_orphans() as u32
}

pub fn part2(input: &[String]) -> u32 {
    let grid = Grid::<u32>::from(input);
    grid.sum_and_multiply_non_orphans() as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        env_logger::builder()
            .format_target(false)
            .format_level(false)
            .format_timestamp(None)
            .init();

        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .map(|l| l.to_string());

        assert_eq!(super::part1(&input), 4361);
    }

    #[test]
    fn part2() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .map(|l| l.to_string());

        assert_eq!(super::part2(&input), 467835);
    }
}
