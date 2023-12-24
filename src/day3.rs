use std::collections::HashMap;
use std::str::Chars;

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

fn read_next_token(s: &Chars) -> (Token, usize) {
    let parsed_num = str::parse::<u32>(
        &s.clone()
            .by_ref()
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
            }
            (Token::Number(n), n_digits)
        }
        Err(_) => {
            let mut s = s.clone();
            let mut c = s.by_ref().next().unwrap();
            if c == '\n' {
                c = s.next().unwrap();
            }
            (Token::Symbol(c), 1)
        }
    }
}

impl Grid {
    fn from(input: &[String]) -> Self {
        let mut nodes: HashMap<Point, Node> = HashMap::new();
        input.iter().enumerate().for_each(|(row_i, row)| {
            let mut col_i = 0;
            let mut s = row.chars();
            while col_i < row.len() {
                let (token, n_digits) = read_next_token(&s);
                log::debug!("Next token: {:?} ({:?} digits)", token, n_digits);

                if let Token::Number(n) = token {
                    if let Some((symbol, pt)) = Self::neighboring_symbol(
                        input,
                        row_i,
                        col_i,
                        col_i + n_digits,
                    ) {
                        log::debug!("Found symbol {:?} at {:?}", symbol, pt);
                        match nodes.get_mut(&pt) {
                            Some(node) => {
                                node.children.push(n);
                            }
                            None => {
                                nodes.insert(
                                    pt,
                                    Node {
                                        symbol,
                                        children: vec![n],
                                    },
                                );
                            }
                        }
                    }
                }

                col_i += n_digits;
                for _ in 0..n_digits {
                    s.next();
                }
            }
        });

        Self { nodes }
    }

    fn neighboring_symbol(
        input: &[String],
        row_i: usize,
        col_i: usize,
        col_end: usize,
    ) -> Option<(char, Point)> {
        let symbol = |(c, pt)| {
            if c == '*'
                || c == '+'
                || c == '&'
                || c == '='
                || c == '$'
                || c == '@'
                || c == '/'
                || c == '-'
                || c == '%'
                || c == '#'
            {
                Some((c, pt))
            } else {
                None
            }
        };

        if let Some(tuple) = input.get(row_i).and_then(|row| {
            (col_i as isize - 1)
                .try_into()
                .ok()
                .and_then(|col_i: usize| {
                    row.chars()
                        .nth(col_i)
                        .and_then(|c| symbol((c, Point { row_i, col_i })))
                })
        }) {
            return Some(tuple);
        }

        if let Some(tuple) = input.get(row_i).and_then(|row| {
            row.chars().nth(col_end).and_then(|c| {
                symbol((
                    c,
                    Point {
                        row_i,
                        col_i: col_end,
                    },
                ))
            })
        }) {
            return Some(tuple);
        }

        let col_i = (col_i as isize - 1).try_into().unwrap_or(0);
        for i in col_i..=col_end {
            if let Some(tuple) = input
                .get((row_i as isize - 1).try_into().unwrap_or(0) as usize)
                .and_then(|row| {
                    row.chars().nth(i).and_then(|c| {
                        symbol((
                            c,
                            Point {
                                row_i: (row_i as isize - 1)
                                    .try_into()
                                    .unwrap_or(0),
                                col_i: i,
                            },
                        ))
                    })
                })
            {
                return Some(tuple);
            }

            if let Some(tuple) = input.get(row_i + 1).and_then(|row| {
                row.chars().nth(i).and_then(|c| {
                    symbol((
                        c,
                        Point {
                            row_i: row_i + 1,
                            col_i: i,
                        },
                    ))
                })
            }) {
                return Some(tuple);
            }
        }

        None
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
