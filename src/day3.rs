use std::str::Chars;

#[derive(Debug)]
struct Node {
    value: u32,
    adjacent_to_symbol: bool,
}

struct Grid {
    nodes: Vec<Node>,
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
            .take_while(|c| c >= &'0' && c <= &'9')
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
        let mut nodes = Vec::new();
        input.iter().enumerate().for_each(|(row_i, row)| {
            let mut col_i = 0;
            let mut s = row.chars();
            while col_i < row.len() {
                let (token, n_digits) = read_next_token(&mut s);
                log::debug!("Next token: {:?} ({:?} digits)", token, n_digits);

                if let Token::Number(n) = token {
                    nodes.push(Node {
                        value: n,
                        adjacent_to_symbol: Self::neighboring_symbol(
                            input,
                            row_i,
                            col_i,
                            col_i + n_digits,
                        ),
                    });
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
    ) -> bool {
        let symbol = |c| {
            c == '*'
                || c == '+'
                || c == '&'
                || c == '='
                || c == '$'
                || c == '@'
                || c == '/'
                || c == '-'
                || c == '%'
                || c == '#'
        };

        if col_i > 0
            && symbol(input[row_i].chars().nth(col_i - 1).unwrap_or(' '))
        {
            return true;
        }

        if symbol(input[row_i].chars().nth(col_end).unwrap_or(' ')) {
            return true;
        }

        let col_i = if col_i > 0 { col_i - 1 } else { col_i };
        for i in col_i..(col_end + 1) {
            if row_i > 0
                && symbol(input[row_i - 1].chars().nth(i).unwrap_or(' '))
            {
                return true;
            }

            if row_i < input.len() - 1
                && symbol(input[row_i + 1].chars().nth(i).unwrap_or(' '))
            {
                return true;
            }
        }

        false
    }

    fn sum_non_orphans(&self) -> u32 {
        self.nodes
            .iter()
            .inspect(|n| log::debug!("{:?}", n))
            .filter(|n| n.adjacent_to_symbol)
            .map(|n| n.value)
            .fold(0, |acc, n| acc + n)
    }
}

pub fn part1(input: &[String]) -> u32 {
    let grid = Grid::from(&input);
    grid.sum_non_orphans()
}

pub fn part2(input: &[String]) -> u32 {
    todo!()
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
}
