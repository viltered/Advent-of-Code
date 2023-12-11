use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day11.txt");
    println!("day 11 part 1: {}", day_11_part_1(input));
    println!("day 11 part 2: {}", day_11_part_2(input));
}

fn solve(input: &str, expansion: usize) -> usize {
    let galaxies: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '#' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .collect();

    let size_y = galaxies.last().expect("No galaxies in map.").1 + 1;
    let size_x = galaxies.iter().map(|g| g.0).max().unwrap() + 1;

    let mut empty_rows: Vec<bool> = vec![true; size_y];
    let mut empty_columns: Vec<bool> = vec![true; size_x];

    galaxies.iter().for_each(|(x, y)| {
        empty_rows[*y] = false;
        empty_columns[*x] = false;
    });

    // get the cumulative distance between rows/columns from the Vec of empty rows/columns
    let get_cumulative_distance = |empty: Vec<bool>| {
        empty
            .into_iter()
            .scan(0, |acc, r| {
                *acc += if r { expansion } else { 1 };
                Some(*acc as isize)
            })
            .collect()
    };
    let cumulative_row_distance: Vec<isize> = get_cumulative_distance(empty_rows);
    let cumulative_column_distance: Vec<isize> = get_cumulative_distance(empty_columns);

    // sum up manhattan distances for each pair of galaxies
    galaxies
        .iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            (cumulative_column_distance[*x2] - cumulative_column_distance[*x1]).abs()
                + (cumulative_row_distance[*y2] - cumulative_row_distance[*y1]).abs()
        })
        .sum::<isize>() as usize
}

fn day_11_part_1(input: &str) -> usize {
    solve(input, 2)
}

fn day_11_part_2(input: &str) -> usize {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use crate::{day_11_part_1, solve};

    const EXAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn day_11_1() {
        assert_eq!(day_11_part_1(EXAMPLE_INPUT), 374);
    }

    #[test]
    fn day_11_2() {
        assert_eq!(solve(EXAMPLE_INPUT, 10), 1030);
    }
}
