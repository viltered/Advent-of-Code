// to convert template to solution file:
// 1. replace 9 with day number
// 2. insert example input
// 3. replace 0 with expected output in tests
// 4. add file with input under /input/day9.txt
// 5. add binary entry for day9.rs in Cargo.toml

use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day9.txt");
    println!("day 9 part 1: {}", day_9_part_1(input));
    println!("day 9 part 2: {}", day_9_part_2(input));
}

/// Build stack of sequences untill the highest sequence consists entirely out of zeros.
fn build_stack(input_line: &str) -> Vec<Vec<i32>> {
    let numbers: Vec<i32> = input_line
        .split_ascii_whitespace()
        .map(|num| num.parse().expect("Not a number."))
        .collect();

    let mut stack: Vec<Vec<i32>> = vec![numbers];

    while stack
        .last()
        .expect("Full pyramid of diffs constructed - no pattern found.")
        .iter()
        .any(|n| *n != 0)
    {
        stack.push(
            stack
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect(),
        )
    }

    stack
}

fn solve(input: &str, is_end: bool) -> i32 {
    let accumulator = if is_end {
        |acc: i32, layer: &Vec<i32>| layer.last().unwrap() + acc
    } else {
        |acc: i32, layer: &Vec<i32>| layer.first().unwrap() - acc
    };

    input
        .lines()
        .map(|line| {
            let stack = build_stack(line);

            stack
                .iter()
                .rev()
                .fold(0, accumulator)
        })
        .sum()
}

fn day_9_part_1(input: &str) -> i32 {
    solve(input, true)
}

fn day_9_part_2(input: &str) -> i32 {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use crate::{day_9_part_1, day_9_part_2};

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn day_9_1() {
        let answer = day_9_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 114);
    }

    #[test]
    fn day_9_2() {
        let answer = day_9_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 2);
    }
}
