use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Card {
    _id: usize,
    winners: HashSet<u32>,
    values: HashSet<u32>,
}

impl Card {
    fn from(s: &str) -> Self {
        let to_set = |s: &str| {
            s.split(' ')
                .filter(|s| !s.is_empty())
                .map(|x| x.parse().expect("invalid number"))
                .collect()
        };
        let split_on_colon: Vec<_> = s.split(':').collect();
        let id = split_on_colon
            .first()
            .expect("Invalid format -- 'Card 1:' is expected")
            .split(' ')
            .last()
            .expect("Invalid format -- 'Card 1:' is expected")
            .parse()
            .expect("Invalid format -- ID must be a number");
        let game: Vec<_> = split_on_colon
            .last()
            .expect("Invalid format -- 'Card 1: 1 2 3 4' is expected")
            .split('|')
            .collect();

        Self {
            _id: id,
            winners: to_set(&game[0]),
            values: to_set(&game[1]),
        }
    }

    fn winners(&self) -> usize {
        self.winners.intersection(&self.values).count()
    }

    fn value(&self) -> u32 {
        let n_winners = self.winners();
        if n_winners == 0 {
            return 0;
        }

        2_u32.pow(n_winners as u32 - 1)
    }
}

pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| Card::from(s))
        .inspect(|c| log::debug!("{:?}", c))
        .map(|c| c.value())
        .inspect(|c| log::debug!("{:?}", c))
        .sum()
}

pub fn part2(input: &[String]) -> u32 {
    let mut counts = vec![1; input.len()];
    input
        .iter()
        .map(|c| Card::from(c).winners())
        .enumerate()
        .for_each(|(i, n)| {
            let count = counts[i];
            counts[i + 1..=i + n].iter_mut().for_each(|c| *c += count);
        });

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .map(|s| s.to_string());

        assert_eq!(super::part1(&input), 13);
    }

    #[test]
    fn part2() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .map(|s| s.to_string());

        assert_eq!(super::part2(&input), 30);
    }
}
