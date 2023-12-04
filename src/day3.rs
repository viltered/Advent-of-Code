use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/day3.txt");
    println!("day 3 part 1: {}", day_3_part_1(input));
    println!("day 3 part 2: {}", day_3_part_2(input));
}

fn day_3_part_1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let number_pattern = Regex::new("\\d+").unwrap();
    let symbol_pattern = Regex::new("[^\\.\\d]").unwrap();

    let mut sum = 0;

    for (index, l) in lines.iter().enumerate() {
        for m in number_pattern.find_iter(l) {
            let mut accepted: bool = false;

            let extended_range =
                m.start().saturating_sub(1)..m.end().saturating_add(1).min(l.len() - 1);
            if index >= 1 {
                accepted |= symbol_pattern.is_match(&lines[index - 1][extended_range.clone()]);
            }
            if index + 1 < lines.len() {
                accepted |= symbol_pattern.is_match(&lines[index + 1][extended_range.clone()])
            }
            accepted |= symbol_pattern.is_match(&l[extended_range]);

            if accepted {
                sum += m.as_str().parse::<u32>().unwrap();
            }
        }
    }

    sum
}

fn day_3_part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    // maps the locations of gears with only one number match so far to said number
    let mut gears: HashMap<(usize, usize), u32> = HashMap::new();

    // regex pattern to match numbers and gears
    let number_pattern = Regex::new("\\d+").unwrap();
    let gear_pattern = Regex::new("\\*").unwrap();

    let mut sum = 0;

    // helper function: finds gears within given range and line index, and processes that they are next to 'number'
    let mut find_gears = |number: u32, index: usize, range: &std::ops::Range<usize>| {
        for gear in gear_pattern.find_iter(&lines[index][range.clone()]) {
            let key = (index, gear.start() + range.start);
            match gears.get(&key) {
                None => {
                    gears.insert(key, number);
                }
                Some(old_number) => {
                    sum += old_number * number;
                    gears.remove(&key);
                }
            }
        }
    };

    // loop over number matches
    for (index, l) in lines.iter().enumerate() {
        for m in number_pattern.find_iter(l) {
            let extended_range =
                m.start().saturating_sub(1)..m.end().saturating_add(1).min(l.len() - 1);

            let number = m.as_str().parse::<u32>().unwrap();

            // same line as number
            find_gears(number, index, &extended_range);
            // line above number
            if index >= 1 {
                find_gears(number, index - 1, &extended_range);
            }
            // line below number
            if index + 1 < lines.len() {
                find_gears(number, index + 1, &extended_range);
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::{day_3_part_1, day_3_part_2};

    const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn day_3_1() {
        let answer = day_3_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 4361);
    }

    #[test]
    fn day_3_2() {
        let answer = day_3_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 467835);
    }
}
