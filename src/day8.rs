use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/day8.txt");
    println!("day 8 part 1: {}", day_8_part_1(input));
    println!("day 8 part 2: {}", day_8_part_2(input));
}

fn parse_input(input: &str) -> (Vec<usize>, HashMap<String, [String; 2]>) {
    let node_pattern = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();

    let mut lines = input.lines();
    let move_pattern: Vec<usize> = lines
        .next()
        .expect("Input empty, invalid.")
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            s => panic!("Unexpected symbol in left/right instructions: '{s}'."),
        })
        .collect();

    let nodes: HashMap<String, [String; 2]> = lines
        .filter_map(|l| node_pattern.captures(l))
        .map(|l| (l[1].to_string(), [l[2].to_string(), l[3].to_string()]))
        .collect();

    (move_pattern, nodes)
}

fn day_8_part_1(input: &str) -> u32 {
    let (move_pattern, nodes) = parse_input(input);

    let mut current_string: &String = &String::from("AAA");
    let end = String::from("ZZZ");

    let count = move_pattern
        .iter()
        .cycle()
        .take_while(|n| {
            current_string = &nodes.get(current_string).expect("Node not in tree.")[**n];
            *current_string != end
        })
        .count();

    count as u32 + 1
}

fn day_8_part_2(input: &str) -> usize {
    let (move_pattern, nodes) = parse_input(input);

    let mut start_strings: Vec<&String> = nodes.keys().filter(|s| s.ends_with('A')).collect();

    // let length = nodes.len();
    // let move_length = move_pattern.len();
    // println!("number of nodes: {length}");
    // println!("number of moves: {move_length}");
    let mut periods: Vec<usize> = vec![0; start_strings.len()];
    let mut first_encounters: Vec<usize> = vec![0; start_strings.len()];

    start_strings
        .iter_mut()
        .zip(first_encounters.iter_mut())
        .zip(periods.iter_mut())
        .for_each(|((start, first_encounter), period)| {
            *first_encounter = move_pattern
                .iter()
                .cycle()
                .take_while(|m| {
                    *start = &nodes.get(*start).expect("Node not in tree.")[**m];
                    !start.ends_with('Z')
                })
                .count() + 1;

            *period = move_pattern
                .iter()
                .cycle()
                .take_while(|m| {
                    *start = &nodes.get(*start).expect("Node not in tree.")[**m];
                    !start.ends_with('Z')
                })
                .count() + 1;
        });

    // The rest of the solution is not general.. it relies on the observation that the
    // time util the first encounter of "..Z" is the same as the period to return to "..Z"
    // for all starting nodes.

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }

    // return LCM of periods
    periods.into_iter().reduce(|a, b| (a * b / gcd(a, b))).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{day_8_part_1, day_8_part_2};

    const EXAMPLE_INPUT_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_INPUT_2: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn day_8_1() {
        assert_eq!(day_8_part_1(EXAMPLE_INPUT_1), 6);
        assert_eq!(day_8_part_1(EXAMPLE_INPUT_2), 2);
    }

    #[test]
    fn day_8_2() {
        assert_eq!(day_8_part_2(EXAMPLE_INPUT_3), 6);
    }
}
