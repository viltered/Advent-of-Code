fn main() {
    let input = include_str!("../input/day1.txt");
    let answer_1 = day_1_part_1(input);
    println!("day 1 part 1: {answer_1}");
    let answer_2 = day_1_part_2(input);
    println!("day 1 part 2: {answer_2}");
}

fn day_1_part_1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut it = line.chars().filter_map(|c| c.to_digit(10));
        let first = it.next().expect("Line doesn't contain any digits.");
        sum += first * 10
            + match it.next_back() {
                Some(last) => last,
                None => first,
            };
    }
    sum
}

const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn day_1_part_2(input: &str) -> u32 {
    let mut sum: usize = 0;

    for line in input.lines() {
        // iterator over the digits written as ascii digit characters and their index of occurrence
        let it_digits = line.chars().enumerate().filter_map(|(index, c)| {
            if c.is_ascii_digit() {
                Some((index, c.to_digit(10).unwrap() as usize))
            } else {
                Option::None
            }
        });

        // iterator over the digits written as words and their index of occurrence
        let it_words = DIGIT_WORDS
            .iter()
            .enumerate()
            .flat_map(|(digit, word)| vec![(line.find(word), digit), (line.rfind(word), digit)])
            .filter_map(|(index, digit)| {
                if index.is_some() {
                    Some((index.unwrap(), digit))
                } else {
                    None
                }
            });

        // both types of matches combined into a single iterator
        let it_matches = it_digits.chain(it_words);

        // get match with highest and lowest index
        let first = it_matches
            .clone()
            .min_by(|(index_1, _), (index_2, _)| index_1.cmp(index_2))
            .unwrap();

        let last = it_matches
            .max_by(|(index_1, _), (index_2, _)| index_1.cmp(index_2))
            .unwrap();

        sum += first.1 * 10 + last.1;

        // // print which characters are selected
        // let first_char = first.1;
        // let last_char = last.1;
        // println!("{line} - {first_char} {last_char}");
        // if last.0 > first.0 {
        //     println!(
        //         "{}^{}^",
        //         " ".repeat(first.0),
        //         " ".repeat(last.0 - first.0 - 1)
        //     );
        // } else {
        //     println!("{}^", " ".repeat(first.0));
        // }
    }

    sum as u32
}

#[cfg(test)]
mod tests {
    use crate::{day_1_part_1, day_1_part_2};

    #[test]
    fn day_1_1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let answer = day_1_part_1(input);
        assert_eq!(answer, 142);
    }
    
    #[test]
    fn day_1_2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let answer = day_1_part_2(input);
        assert_eq!(answer, 281);
    }
    
    #[test]
    fn day_1_2_failed() {
        // test case from regular input which failed before
        let answer = day_1_part_2("kpzfgpxdonesix2fourninefourfour");
        assert_eq!(answer, 14);
    }
}

