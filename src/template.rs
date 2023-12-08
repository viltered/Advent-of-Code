
// to convert template to solution file:
// 1. replace Z with day number
// 2. insert example input
// 3. replace 0 with expected output in tests
// 4. add file with input under /input/dayZ.txt
// 5. add binary entry for dayZ.rs in Cargo.toml

fn main() {
    let input = include_str!("../input/dayZ.txt");
    println!("day Z part 1: {}", day_Z_part_1(input));
    println!("day Z part 2: {}", day_Z_part_2(input));
}

fn day_Z_part_1(input: &str) -> u32 {
    todo!()
}

fn day_Z_part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{day_Z_part_1, day_Z_part_2};

    const EXAMPLE_INPUT: &str = "";

    #[test]
    fn day_Z_1() {
        let answer = day_Z_part_1(EXAMPLE_INPUT);
        assert_eq!(answer, 0);
    }

    #[test]
    fn day_Z_2() {
        let answer = day_Z_part_2(EXAMPLE_INPUT);
        assert_eq!(answer, 0);
    }
}
