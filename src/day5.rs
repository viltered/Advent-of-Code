use itertools::Itertools;
use regex::Regex;
use std::ops::Range;

fn main() {
    let input = include_str!("../input/day5.txt");
    println!("day 5 part 1: {}", day_5_part_1(input));
    println!("day 5 part 2: {}", day_5_part_2(input));
}

fn day_5_part_1(input: &str) -> u32 {
    let mut lines = input.lines();
    let map_line_pattern = Regex::new(r"^(\d+) +(\d+) +(\d+)$").unwrap();

    // parse seeds on the first line.
    // each value gets a paired bool to indicate whether it has been mapped yet using the currently parsed map
    let seeds_str = lines.next().unwrap();
    let mut seeds: Vec<(bool, i64)> = seeds_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| (true, s.parse().unwrap()))
        .collect();

    // apply maps on remaining lines
    for line in lines {
        if line.ends_with("map:") {
            // when a new map starts, all values regain the ability to be mapped
            seeds.iter_mut().for_each(|item| item.0 = true);
        } else if let Some(captures) = map_line_pattern.captures(line) {
            // if the regex for a map entry matches, extract the source range and offset
            let destination_range_start: i64 = captures[1].parse().unwrap();
            let source_range_start: i64 = captures[2].parse().unwrap();
            let range_length: i64 = captures[3].parse().unwrap();

            let source_range = source_range_start..(source_range_start + range_length);
            let offset: i64 = destination_range_start - source_range_start;

            // apply offset to all values not already mapped and in range
            seeds
                .iter_mut()
                .filter(|s| s.0 && source_range.contains(&s.1))
                .for_each(|s| *s = (false, s.1 + offset));
        }
    }

    seeds.iter().map(|s| s.1).min().unwrap() as u32
}

#[derive(PartialEq, Debug)]
enum RangeOverlap<T> {
    None, // the range is entirely outside the source range
    Full, // the range is entirely within the source range
    Partial {
        overlapping: Range<T>,
        not_overlapping: Range<T>,
    }, // the range overlaps the source range on one side
    Eclipse {
        overlapping: Range<T>,
        not_overlapping_left: Range<T>,
        not_overlapping_right: Range<T>,
    }, // the range eclipses the source range on both ends
}

fn find_range_overlap(source_range: &Range<i64>, range: &Range<i64>) -> RangeOverlap<i64> {
    if range.start >= source_range.end {
        RangeOverlap::None
    } else if range.start >= source_range.start {
        if range.end <= source_range.end {
            RangeOverlap::Full
        } else {
            RangeOverlap::Partial {
                overlapping: range.start..source_range.end,
                not_overlapping: source_range.end..range.end,
            }
        }
    } else {
        if range.end <= source_range.start {
            RangeOverlap::None
        } else if range.end < source_range.end {
            RangeOverlap::Partial {
                overlapping: source_range.start..range.end,
                not_overlapping: range.start..source_range.start,
            }
        } else {
            RangeOverlap::Eclipse {
                overlapping: source_range.clone(),
                not_overlapping_left: range.start..source_range.start,
                not_overlapping_right: source_range.end..range.end,
            }
        }
    }
}

fn day_5_part_2(input: &str) -> u32 {
    let mut lines = input.lines();
    let map_line_pattern = Regex::new(r"^(\d+) +(\d+) +(\d+)$").unwrap();

    // parse seeds on the first line
    // each value gets a paired bool to indicate whether it has been mapped yet using the currently parsed map
    let seeds_str = lines.next().unwrap();
    let mut seeds: Vec<(bool, Range<i64>)> = seeds_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .tuples()
        .map(|(a, b)| (true, a..(a + b)))
        .collect();

    // apply maps on remaining lines
    for line in lines {
        if line.ends_with("map:") {
            // when a new map starts, all values regain the ability to be mapped
            seeds.iter_mut().for_each(|item| item.0 = true);
        } else if let Some(captures) = map_line_pattern.captures(line) {
            // if the regex for a map entry matches, extract the source range and offset
            let destination_range_start: i64 = captures[1].parse().unwrap();
            let source_range_start: i64 = captures[2].parse().unwrap();
            let range_length: i64 = captures[3].parse().unwrap();

            let source_range = source_range_start..(source_range_start + range_length);
            let offset: i64 = destination_range_start - source_range_start;

            let mut new_range_buffer: Vec<(bool, Range<i64>)> = Vec::new();

            seeds.iter_mut().filter(|s| s.0).for_each(|s| {
                match find_range_overlap(&source_range, &s.1) {
                    RangeOverlap::None => {}
                    RangeOverlap::Full => {
                        s.0 = false;
                        s.1 = (s.1.start + offset)..(s.1.end + offset);
                    }
                    RangeOverlap::Partial {
                        overlapping,
                        not_overlapping,
                    } => {
                        s.0 = false;
                        s.1 = (overlapping.start + offset)..(overlapping.end + offset);
                        new_range_buffer.push((true, not_overlapping));
                    }
                    RangeOverlap::Eclipse {
                        overlapping,
                        not_overlapping_left,
                        not_overlapping_right,
                    } => {
                        s.0 = false;
                        s.1 = (overlapping.start + offset)..(overlapping.end + offset);
                        new_range_buffer.push((true, not_overlapping_left));
                        new_range_buffer.push((true, not_overlapping_right));
                    }
                }
            });

            seeds.append(&mut new_range_buffer);
        }
    }

    seeds.iter().map(|s| s.1.start).min().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use crate::{day_5_part_1, day_5_part_2, find_range_overlap, RangeOverlap};

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn day_5_1() {
        let answer = day_5_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 35);
    }

    #[test]
    fn day_5_2_range_overlap_finder() {
        assert_eq!(
            find_range_overlap(&(0..5), &(2..7)),
            RangeOverlap::Partial {
                overlapping: 2..5,
                not_overlapping: 5..7
            }
        );
        assert_eq!(find_range_overlap(&(0..7), &(2..5)), RangeOverlap::Full);
        assert_eq!(
            find_range_overlap(&(2..5), &(0..7)),
            RangeOverlap::Eclipse {
                overlapping: 2..5,
                not_overlapping_left: 0..2,
                not_overlapping_right: 5..7
            }
        );
        assert_eq!(find_range_overlap(&(2..4), &(4..9)), RangeOverlap::None);
        assert_eq!(find_range_overlap(&(4..8), &(0..4)), RangeOverlap::None);
    }

    #[test]
    fn day_5_2() {
        let answer = day_5_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 46);
    }
}
