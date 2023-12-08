use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day7.txt");
    println!("day 7 part 1: {}", day_7_part_1(input));
    println!("day 7 part 2: {}", day_7_part_2(input));
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash, Debug)]
enum Card {
    Joker,
    Number(u8),
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl Hand {
    fn new(s: &str, is_j_joker: bool) -> Self {
        assert!(s.len() == 5);
        // parse the five cards
        let cards: Vec<Card> = s
            .chars()
            .map(|c| match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' if is_j_joker => Card::Joker,
                'J' if !is_j_joker => Card::J,
                'T' => Card::T,
                '2'..='9' => Card::Number(c.to_string().parse().unwrap()),
                _ => panic!("Invalid card symbol"),
            })
            .take(5)
            .collect();

        // construct Vec with number of cards, by most occurring to least occurring

        // to count groups using simple counting method assuming there are no jokers (part 1)
        // cards.iter().counts().into_values().sorted().rev().collect()

        // count groups and add jokers to largest group
        let mut count_map = cards.iter().counts();
        let jokers = count_map.remove(&Card::Joker).unwrap_or(0);
        let mut counts: Vec<usize> = count_map.into_values().sorted_by(|a, b| b.cmp(a)).collect();
        if counts.is_empty() {
            counts.push(jokers);
        } else {
            counts[0] += jokers;
        }

        // determine the type of hand by inspecting the counts
        let hand_type = match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Hand { hand_type, cards }
    }
}

fn day_7_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_ascii_whitespace();
            (
                Hand::new(it.next().expect("Invalid line"), false),
                it.next().expect("Missing bet").parse::<u32>().unwrap(),
            )
        })
        .sorted()
        .zip(1..)
        // .inspect(|((hand, bet), rank)| println!("{rank}. {hand:?} {bet:?}"))
        .map(|((_hand, bet), rank)| rank * bet)
        .sum()
}

fn day_7_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_ascii_whitespace();
            (
                Hand::new(it.next().expect("Invalid line"), true),
                it.next().expect("Missing bet").parse::<u32>().unwrap(),
            )
        })
        .sorted()
        .zip(1..)
        // .inspect(|((hand, bet), rank)| println!("{rank}. {hand:?} {bet:?}"))
        .map(|((_hand, bet), rank)| rank * bet)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{day_7_part_1, day_7_part_2};

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn day_7_1() {
        let answer = day_7_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 6440);
    }

    #[test]
    fn day_7_2() {
        let answer = day_7_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 5905);
    }
}
