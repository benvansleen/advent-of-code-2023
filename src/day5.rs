use std::collections::{BTreeMap, HashMap};
use std::ops::Range;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct Map {
    from: String,
    to: String,
    from_ranges: BTreeMap<u64, Range<u64>>,
    to_ranges: Vec<Range<u64>>,
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
            let mut vals: Vec<u64> = r
                .split(' ')
                .map(|v| v.parse())
                .collect::<Result<_, _>>()
                .ok()?;
            let range = vals.pop()?;
            let from_begin = vals.pop()?;
            let to_begin = vals.pop()?;

            from_ranges.push(from_begin..(from_begin + range));
            to_ranges.push(to_begin..(to_begin + range));
            Some(())
        });

        let mut zipped: Vec<_> =
            to_ranges.iter().zip(from_ranges.iter()).collect();
        zipped.sort_by_key(|(_, from)| from.start);
        let to_ranges = zipped.into_iter().map(|(to, _)| to.clone()).collect();

        Some(Map {
            from,
            to,
            from_ranges: from_ranges
                .into_iter()
                .map(|r| (r.start, r))
                .collect(),
            to_ranges,
        })
    }

    fn next_val(&self, val: u64) -> u64 {
        match self
            .from_ranges
            .range(..=val)
            .zip(self.to_ranges.iter())
            .last()
        {
            Some(((start, range), to_range)) if range.contains(&val) => {
                let offset = val - start;
                to_range.start + offset
            }
            _ => val,
        }
    }
}

fn get_seed_list_from_groups(groups: &[&str]) -> Option<Vec<u64>> {
    groups
        .first()?
        .trim()
        .split(':')
        .last()?
        .trim()
        .split(' ')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()
        .ok()
}

pub fn part1(input: &[String]) -> u32 {
    let input = input.join("\n");
    let groups: Vec<_> = input.split("\n\n").collect();
    let seed_list = get_seed_list_from_groups(&groups).unwrap();
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
                log::debug!("type: {:?}, val: {:?}", next_type, next_val);
                next_type = &map.to;
                next_val = map.next_val(next_val);
            }
            next_val
        })
        .min()
        .unwrap() as u32
}

fn get_seed_ranges_from_groups(groups: &[&str]) -> Option<Vec<Range<u64>>> {
    let ranges = groups
        .first()?
        .trim()
        .split(':')
        .last()?
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok());

    let start_values = ranges.clone().step_by(2);
    let offset_values = ranges.clone().skip(1).step_by(2);
    Some(
        start_values
            .zip(offset_values)
            .map(|(start, offset)| start..(start + offset))
            .collect(),
    )
}

fn distribute_seeds<F>(
    l: Vec<Range<u64>>,
    lookup_table: HashMap<String, Map>,
    f: F,
) -> u64
where
    F: Fn(u64, &HashMap<String, Map>) -> u64 + Copy + Send + 'static,
{
    let lookup_table = Arc::new(lookup_table);
    let l = Box::leak(l.into_boxed_slice());

    let batch_size = 10_000;
    let (task_tx, task_rx) = crossbeam_channel::bounded(10 * batch_size);
    let producer = std::thread::spawn(move || {
        l.iter_mut().for_each(|seed_range| loop {
            let next_batch: Vec<_> = seed_range.take(batch_size).collect();
            if next_batch.is_empty() {
                break;
            }
            task_tx.send(next_batch).unwrap();
        });

        drop(task_tx);
        log::debug!("producer done");
    });

    let n_threads: usize = std::thread::available_parallelism().unwrap().into();
    let (tx, rx) = crossbeam_channel::unbounded();
    let threads: Vec<_> = (0..n_threads)
        .map(|_| {
            let task_rx = task_rx.clone();
            let tx = tx.clone();
            let lookup_table = Arc::clone(&lookup_table);
            std::thread::spawn(move || {
                while let Ok(seeds) = task_rx.recv() {
                    let local_min = seeds
                        .into_iter()
                        .map(|seed| f(seed, &lookup_table))
                        .min()
                        .unwrap_or(u64::MAX);
                    tx.send(local_min).unwrap();
                    log::debug!("sending local min: {}", local_min);
                }
                drop(tx);
                log::debug!("worker done");
            })
        })
        .collect();
    drop(tx);

    let smallest_loc = rx.iter().min().unwrap();
    producer.join().unwrap();
    threads.into_iter().for_each(|t| t.join().unwrap());
    smallest_loc
}

pub fn part2(input: &[String]) -> u32 {
    let input = input.join("\n");
    let groups: Vec<_> = input.split("\n\n").collect();
    let seed_list = get_seed_ranges_from_groups(&groups).unwrap();
    let lookup: HashMap<_, Map> = HashMap::from_iter(
        groups[1..]
            .iter()
            .filter_map(|g| Map::from(g))
            .map(|m| (m.from.clone(), m)),
    );

    let process_seed = |seed, lookup: &HashMap<_, Map>| {
        let mut next_val = seed;
        let mut next_type = "seed";
        while let Some(map) = lookup.get(next_type) {
            log::debug!("type: {}, val: {}", next_type, next_val);
            next_type = &map.to;
            next_val = map.next_val(next_val);
        }
        next_val
    };

    distribute_seeds(seed_list, lookup, process_seed).try_into().unwrap()
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

    #[test]
    fn part2() {
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

        assert_eq!(super::part2(&input), 46);
    }
}
