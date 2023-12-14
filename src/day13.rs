fn main() {
    let input = include_str!("../input/day13.txt");
    println!("day 13 part 1: {}", day_13_part_1(input));
    println!("day 13 part 2: {}", day_13_part_2(input));
}

fn parse_grid(grid: &str) -> Vec<Vec<char>> {
    grid.lines().map(|line| line.chars().collect()).collect()
}

// find the value for a mirror for which the reflection is wrong up to the given number of smudges
fn find_smudge_value(grid: Vec<Vec<char>>, smudges: usize) -> usize {
    let size_y = grid.len();
    let size_x = grid.first().expect("Empty grid.").len();

    // find horizontal symmetry
    for mirror_index in 1..size_x {
        let min_index = mirror_index.saturating_sub(size_x.saturating_sub(mirror_index));
        let max_index = (2 * mirror_index).min(size_x);
        if smudges
            == grid
                .iter()
                .map(|line| {
                    line[min_index..mirror_index]
                        .iter()
                        .zip(line[mirror_index..max_index].iter().rev())
                        .filter(|(&c, &m_c)| c != m_c)
                        .count()
                })
                .sum()
        {
            return mirror_index;
        }
    }

    // find vertical symmetry
    for mirror_index in 1..size_y {
        let min_index = mirror_index.saturating_sub(size_y.saturating_sub(mirror_index));
        let max_index = (2 * mirror_index).min(size_y);

        if smudges
            == grid[min_index..mirror_index]
                .iter()
                .zip(grid[mirror_index..max_index].iter().rev())
                .map(|(line, m_line)| {
                    line.iter()
                        .zip(m_line.iter())
                        .filter(|(&c, &m_c)| c != m_c)
                        .count()
                })
                .sum()
        {
            return 100 * mirror_index;
        }
    }

    0
}

fn day_13_part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .map(|grid_str| find_smudge_value(parse_grid(grid_str), 0))
        .sum()
}

fn day_13_part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .into_iter()
        .map(|grid_str| find_smudge_value(parse_grid(grid_str), 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{day_13_part_1, day_13_part_2};

    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn day_13_1() {
        assert_eq!(day_13_part_1(EXAMPLE_INPUT), 405);
    }

    #[test]
    fn day_13_2() {
        assert_eq!(day_13_part_2(EXAMPLE_INPUT), 400);
    }
}
