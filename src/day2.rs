use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/day2.txt");
    println!("day 2 part 1: {}", day_2_part_1(input));
    println!("day 2 part 2: {}", day_2_part_2(input));
}

fn day_2_part_1(input: &str) -> u32 {
    let max_color = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut sum = 0;

    for line in input.lines() {
        let (game_id, record) = line.split_once(": ").expect("Incorrect format.");
        let game_id = game_id
            .trim_start_matches("Game ")
            .parse::<u32>()
            .expect("Non-numeric id.");

        let accepted = record.split("; ").flat_map(|r| r.split(", ")).all(|r| {
            let (num, color) = r.split_once(' ').expect("Incorrect format");
            max_color[color] >= num.parse::<u32>().expect("Non-numeric number of cubes.")
        });

        if accepted {
            sum += game_id;
        }
    }
    sum
}

fn day_2_part_2(input: &str) -> u32 {
    let mut sum = 0;
    let mut current_minimum = HashMap::new();

    for line in input.lines() {
        current_minimum.extend([("red", 0), ("green", 0), ("blue", 0)]);

        let (_, record) = line.split_once(": ").expect("Incorrect format.");

        record
            .split("; ")
            .flat_map(|r| r.split(", "))
            .for_each(|r| {
                let (num, color) = r.split_once(' ').expect("Incorrect format");
                let num = num.parse::<u32>().expect("Non-numeric number of cubes.");
                current_minimum
                    .insert(color, *current_minimum.get(color).unwrap().max(&num));
            });

        sum += current_minimum.drain().fold(1, |old, new| old * new.1);
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::{day_2_part_1, day_2_part_2};

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn day_2_1() {
        let answer = day_2_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 8);
    }

    #[test]
    fn day_2_2() {
        let answer = day_2_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 2286);
    }
}
