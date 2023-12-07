use regex::Regex;
use std::collections::{BTreeSet, VecDeque};

fn main() {
    let input = include_str!("../input/day4.txt");
    println!("day 4 part 1: {}", day_4_part_1(input));
    println!("day 4 part 2: {}", day_4_part_2(input));
}

fn day_4_part_1(input: &str) -> u32 {
    let line_pattern = Regex::new(r"^Card +\d+: ([\d ]+) \| ([\d ]+)$").unwrap();
    let mut total_score: u32 = 0;

    for line in input.lines() {
        // parse numbers
        let captures = line_pattern.captures(line).expect("Incorrect line format");
        let winning_numbers_str = &captures[1];
        let numbers_you_have_str = &captures[2];

        // store winning numbers in an cheaply searchable set
        let winning_numbers: BTreeSet<u32> = winning_numbers_str
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        // compute amount of winning numbers
        let wins = numbers_you_have_str
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .filter(|n| winning_numbers.contains(n))
            .count();

        // update score
        total_score += 1 << wins >> 1;
    }

    total_score
}

fn day_4_part_2(input: &str) -> u32 {
    let line_pattern = Regex::new(r"^Card +\d+: ([\d ]+) \| ([\d ]+)$").unwrap();
    let mut total_score: u32 = 0;
    let mut future_copies: VecDeque<u32> = VecDeque::new();

    for line in input.lines() {
        // parse numbers
        let captures = line_pattern.captures(line).expect("Incorrect line format");
        let winning_numbers_str = &captures[1];
        let numbers_you_have_str = &captures[2];

        // store winning numbers in an cheaply searchable set
        let winning_numbers: BTreeSet<u32> = winning_numbers_str
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        // compute amount of winning numbers
        let wins = numbers_you_have_str
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .filter(|n| winning_numbers.contains(n))
            .count();

        // increase score by number of copies of current card
        let current_copies = 1 + future_copies.pop_front().unwrap_or(0);
        total_score += current_copies;

        // increase copy count for future cards, wether already present in the VecDeque or not
        let futures_present = wins.min(future_copies.len());
        for item in future_copies.iter_mut().take(futures_present) {
            *item += current_copies;
        }
        for _ in futures_present..wins {
            future_copies.push_back(current_copies);
        }
    }

    total_score
}

#[cfg(test)]
mod tests {
    use crate::{day_4_part_1, day_4_part_2};

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn day_4_1() {
        let answer = day_4_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 13);
    }

    #[test]
    fn day_4_2() {
        let answer = day_4_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 30);
    }
}
