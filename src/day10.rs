fn main() {
    let input = include_str!("../input/day10.txt");
    println!("day 10 part 1: {}", day_10_part_1(input));
    println!("day 10 part 2: {}", day_10_part_2(input));
}

#[derive(Clone, Debug)]
struct Offset(i32, i32);

fn next_pipe_offset(offset: &Offset, pipe: &char) -> Option<Offset> {
    match (pipe, offset) {
        ('F', Offset(-1, 0)) => Some(Offset(0, 1)),
        ('F', Offset(0, -1)) => Some(Offset(1, 0)),
        ('7', Offset(1, 0)) => Some(Offset(0, 1)),
        ('7', Offset(0, -1)) => Some(Offset(-1, 0)),
        ('J', Offset(0, 1)) => Some(Offset(-1, 0)),
        ('J', Offset(1, 0)) => Some(Offset(0, -1)),
        ('L', Offset(0, 1)) => Some(Offset(1, 0)),
        ('L', Offset(-1, 0)) => Some(Offset(0, -1)),
        ('-', Offset(x, 0)) => Some(Offset(*x, 0)),
        ('|', Offset(0, y)) => Some(Offset(0, *y)),
        // + is a sentinel-like special value; only used to enter the main loop.
        // In retrospect, replacing 'S' with its proper pipe symbol and storing the
        // start location separately could be a simpler solution.
        ('+', offset) => Some((*offset).clone()),
        _ => None,
    }
}

fn find_start(grid: &[Vec<char>]) -> (i32, i32) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No start found.");
}

/// Finds initial offset to move out of S.
/// Guarantees that the offset will be the up direction (0, -1) if possible.
fn find_initial_offset(start: (i32, i32), grid: &Vec<Vec<char>>) -> Offset {
    let size_y = grid.len() as i32;
    let size_x = grid.first().expect("Empty grid!").len() as i32;

    for test_offset in [Offset(0, -1), Offset(0, 1), Offset(1, 0), Offset(-1, 0)] {
        let new_x = (start.0 + test_offset.0).clamp(0, size_x);
        let new_y = (start.1 + test_offset.1).clamp(0, size_y);
        let cell = grid[new_y as usize][new_x as usize];
        if next_pipe_offset(&test_offset, &cell).is_some() {
            return test_offset;
        }
    }
    panic!("No initial step from S leads to a path.");
}

fn day_10_part_1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (start_x, start_y) = find_start(&grid);

    let mut current_offset = find_initial_offset((start_x, start_y), &grid);
    let (mut current_x, mut current_y): (i32, i32) = (start_x, start_y);
    let mut current_char = '+';

    // follow the loop back to the start, while tracking the number of pipes
    let mut count = 0;
    while current_char != 'S' {
        current_offset = next_pipe_offset(&current_offset, &current_char)
            .expect("Pipe dead end!");
        current_x += current_offset.0;
        current_y += current_offset.1;
        current_char = grid[current_y as usize][current_x as usize];
        count += 1;
    }

    count / 2
}

fn day_10_part_2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let size_y = grid.len();
    let size_x = grid[0].len();
    let mut loop_walls: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    let (start_x, start_y) = find_start(&grid);

    let start_offset = find_initial_offset((start_x, start_y), &grid);
    let mut current_offset = start_offset.clone();
    let mut next_offset: Offset;
    let (mut current_x, mut current_y): (i32, i32) = (start_x, start_y);
    let mut current_char = '+';
    // println!("starting pos: ({start_x},{start_y})");

    // follow the path, keeping track of:
    //   1) which tiles are part of the loop (loop_walls)
    //   2) which tiles have a pipe going up (L, |, J or S depending on starting offset)
    while current_char != 'S' {
        // println!("  pos: ({current_x} {current_y}) {current_char} -  {current_offset:?}");
        next_offset = next_pipe_offset(&current_offset, &current_char).unwrap_or_else(|| 
            panic!("Pipe dead end! ({current_x},{current_y}) {current_offset:?} {current_char}")
        );
        current_offset = next_offset;
        current_x += current_offset.0;
        current_y += current_offset.1;
        current_char = grid[current_y as usize][current_x as usize];
        loop_walls[current_y as usize][current_x as usize] = match current_char {
            'L' | '|' | 'J' => true,
            'S' if start_offset.1 == -1 => true,
            _ => false,
        };
    }

    let mut is_inside = false;

    // find which corner points, each located in between 4 tiles, are inside the loop. 
    let corner_points_inside: Vec<Vec<bool>> = loop_walls
        .iter()
        .map(|row| {
            row.iter()
                .map(|w| {
                    is_inside ^= w;
                    is_inside
                })
                .collect()
        })
        .collect();

    // print corner points inside the loop
    // println!("{}", input);
    // corner_points_inside.iter().for_each(|row| println!("{}", row.iter().map(|p| if *p {'#'} else {' '}).collect::<String>()));

    // count tiles with all four corner points inside the loop, meaning the tile itself is in the loop
    let mut count = 0;
    for x in 0..size_x - 1 {
        for y in 0..size_y - 1 {
            if corner_points_inside[y][x]
                && corner_points_inside[y + 1][x]
                && corner_points_inside[y][x + 1]
                && corner_points_inside[y + 1][x + 1]
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{day_10_part_1, day_10_part_2};

    const SIMPLE_LOOP: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const COMPLEX_LOOP: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn day_10_1() {
        assert_eq!(day_10_part_1(SIMPLE_LOOP), 4);
        assert_eq!(day_10_part_1(COMPLEX_LOOP), 8);
    }

    const SIMPLE_FILL: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn day_10_2() {
        assert_eq!(day_10_part_2(SIMPLE_FILL), 4);
        assert_eq!(day_10_part_2(SIMPLE_LOOP), 1);
        assert_eq!(day_10_part_2(COMPLEX_LOOP), 1);
    }
}
