
// to convert template to solution file:
// 1. replace X with day number
// 2. insert example input
// 3. replace 0 with expected output in tests
// 4. add file with input under /input/dayX.txt
// 5. add binary entry for dayX.rs in Cargo.toml

fn main() {
    let input = include_str!("../input/dayX.txt");
    println!("day X part 1: {}", day_X_part_1(input));
    println!("day X part 2: {}", day_X_part_2(input));
}

fn day_X_part_1(input: &str) -> u32 {
    todo!()
}

fn day_X_part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{day_X_part_1, day_X_part_2};

    const EXAMPLE_INPUT: &str = "";

    #[test]
    fn day_X_1() {
        let answer = day_X_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 0);
    }

    #[test]
    fn day_X_2() {
        let answer = day_X_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 0);
    }
}
