fn main() {
    let input = include_str!("../input/day6.txt");
    println!("day 6 part 1: {}", day_6_part_1(input));
    println!("day 6 part 2: {}", day_6_part_2(input));
}

fn ways_to_win(time: &f64, top_score: &f64) -> u64 {
    let root = (time * time - 4.0 * top_score).sqrt();
    let min_boost_time: f64 = 0.5 * (time - root);
    let max_boost_time: f64 = 0.5 * (time + root);
    max_boost_time.ceil() as u64 - min_boost_time.floor() as u64 - 1
}

/// Same as ways_to_win(), but with the arithmetic shifted around to avoid overflow.
fn ways_to_win_centered(time: &f64, top_score: &f64) -> u64 {
    let root = (time * time - 4.0 * top_score).sqrt();
    let min_boost_time: f64 = 0.5 * (time - root);
    let max_boost_time = time - min_boost_time;

    max_boost_time.ceil() as u64 - min_boost_time.floor() as u64 - 1
}

fn day_6_part_1(input: &str) -> u64 {
    let mut parsed_lines = input.lines().map(|l| {
        l.split_ascii_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect()
    });

    let times: Vec<f64> = parsed_lines.next().unwrap();
    let top_scores: Vec<f64> = parsed_lines.next().unwrap();

    times
        .iter()
        .zip(top_scores.iter())
        .map(|(time, top_score)| ways_to_win(time, top_score))
        .product()
}

fn day_6_part_2(input: &str) -> u64 {
    let mut parsed_lines = input.lines().map(|l| {
        l.split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap()
    });
    let time = parsed_lines.next().unwrap();
    let top_score = parsed_lines.next().unwrap();

    println!("time: {time}   top_score: {top_score}");

    ways_to_win_centered(&time, &top_score)
}

#[cfg(test)]
mod tests {
    use crate::{day_6_part_1, day_6_part_2};

    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn day_6_1() {
        let answer = day_6_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 288);
    }

    #[test]
    fn day_6_2() {
        let answer = day_6_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 71503);
    }
}
