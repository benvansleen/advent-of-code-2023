use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    from_ranges: Vec<Range<usize>>,
    to_ranges: Vec<Range<usize>>,
}

impl Map {
    fn from(s: &str) -> Option<Map> {
        let split_on_colon: Vec<_> = s.split(':').collect();

        let label: Vec<_> = split_on_colon
            .first()?
            .trim()
            .split("map")
            .collect::<Vec<_>>()
            .first()?
            .trim()
            .split("-to-")
            .collect();

        let from = label.first()?.trim().to_string();
        let to = label.last()?.trim().to_string();

        let mut ranges = split_on_colon.last()?.trim().split('\n');

        let mut from_ranges = Vec::new();
        let mut to_ranges = Vec::new();
        ranges.try_for_each(|r| {
            let mut vals: Vec<usize> =
                r.split(' ').map(|v| v.parse().unwrap()).collect();
            let range = vals.pop()?;
            let from_begin = vals.pop()?;
            let to_begin = vals.pop()?;

            from_ranges.push(from_begin..(from_begin + range));
            to_ranges.push(to_begin..(to_begin + range));
            Some(())
        });

        Some(Map {
            from,
            to,
            from_ranges,
            to_ranges,
        })
    }

    fn next_val(&self, val: usize) -> usize {
        let mut next_val = val;
        for (from_range, to_range) in
            self.from_ranges.iter().zip(self.to_ranges.iter())
        {
            if from_range.contains(&val) {
                let offset = val - from_range.start;
                next_val = to_range.start + offset;
                break;
            }
        }
        next_val
    }
}

fn get_seed_list_from_groups(groups: &Vec<&str>) -> Vec<usize> {
    groups
        .first()
        .unwrap()
        .trim()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part1(input: &[String]) -> u32 {
    let input = input.join("\n");
    let groups: Vec<_> = input.split("\n\n").collect();
    let seed_list = get_seed_list_from_groups(&groups);
    let lookup: HashMap<_, Map> = HashMap::from_iter(
        groups[1..]
            .iter()
            .filter_map(|g| Map::from(g))
            .map(|m| (m.from.clone(), m)),
    );

    seed_list
        .into_iter()
        .map(|seed| {
            let mut next_val = seed;
            let mut next_type = "seed";
            while let Some(map) = lookup.get(next_type) {
                log::debug!("type: {}, val: {}", next_type, next_val);
                next_type = &map.to;
                next_val = map.next_val(next_val);
            }
            next_val
        })
        .min()
        .unwrap() as u32

}

pub fn part2(input: &[String]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .trim()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        assert_eq!(super::part1(&input), 35);
    }
}
