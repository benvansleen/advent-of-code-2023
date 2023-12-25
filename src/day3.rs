use core::slice::Iter;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    symbol: char,
    children: Vec<u32>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    row_i: usize,
    col_i: usize,
}

struct Grid {
    nodes: HashMap<Point, Node>,
}

#[derive(Debug)]
enum Token {
    Number(u32),
    Symbol(char),
}

fn read_next_token(s: &mut Iter<char>) -> (Token, usize) {
    let parsed_num: Result<u32, _> = str::parse(
        &s.clone()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>(),
    );
    match parsed_num {
        Ok(n) => {
            let mut n_tmp = n;
            let mut n_digits = 0;
            while n_tmp > 0 {
                n_digits += 1;
                n_tmp /= 10;
                s.next();
            }
            (Token::Number(n), n_digits)
        }
        Err(_) => (Token::Symbol(*s.next().unwrap()), 1),
    }
}

impl Grid {
    fn from(input: &[String]) -> Self {
        let mut nodes: HashMap<Point, Node> = HashMap::new();
        let input: Vec<Vec<char>> =
            input.iter().map(|s| s.chars().collect()).collect();

        input.iter().enumerate().for_each(|(row_i, row)| {
            let mut col_i = 0;
            let mut s = row.iter();
            while col_i < row.len() {
                let (token, n_digits) = read_next_token(&mut s);
                log::debug!("Next token: {:?} ({:?} digits)", token, n_digits);
                if let Token::Number(n) = token {
                    if let Some((symbol, pt)) = Self::neighboring_symbol(
                        &input,
                        row_i,
                        col_i,
                        col_i + n_digits,
                    ) {
                        log::debug!("Found symbol {:?} at {:?}", symbol, pt);
                        nodes
                            .entry(pt)
                            .or_insert(Node {
                                symbol,
                                children: Vec::new(),
                            })
                            .children
                            .push(n);
                    }
                }

                col_i += n_digits;
            }
        });

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

    fn sum_non_orphans(&self) -> u32 {
        self.nodes
            .values()
            .inspect(|n| log::debug!("{}: {:?}", n.symbol, n.children))
            .map(|n| n.children.iter().sum::<u32>())
            .sum()
    }

    fn sum_and_multiply_non_orphans(&self) -> u32 {
        self.nodes
            .values()
            .filter(|node| node.symbol == '*' && node.children.len() == 2)
            .inspect(|node| log::debug!("{}: {:?}", node.symbol, node.children))
            .map(|node| node.children.iter().product::<u32>())
            .sum()
    }
}

pub fn part1(input: &[String]) -> u32 {
    let grid = Grid::from(input);
    grid.sum_non_orphans()
}

pub fn part2(input: &[String]) -> u32 {
    let grid = Grid::from(input);
    grid.sum_and_multiply_non_orphans()
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
