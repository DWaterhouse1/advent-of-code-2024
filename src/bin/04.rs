advent_of_code::solution!(4);

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy)]
struct GridDimensions {
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, EnumIter, Display)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

fn traverse(idx: usize, dims: GridDimensions, direction: Direction) -> Option<usize> {
    let end_idx = dims.width * dims.height;
    let row = idx / dims.width;
    let column = idx % dims.width;

    match direction {
        Direction::Up => (row > 0).then(|| idx - dims.width),

        Direction::UpRight => {
            traverse(idx, dims, Direction::Up).and_then(|x| traverse(x, dims, Direction::Right))
        }

        Direction::Right => (column < dims.width - 1).then_some(idx + 1),

        Direction::DownRight => {
            traverse(idx, dims, Direction::Down).and_then(|x| traverse(x, dims, Direction::Right))
        }

        Direction::Down => (idx + dims.width < end_idx).then_some(idx + dims.width),

        Direction::DownLeft => {
            traverse(idx, dims, Direction::Down).and_then(|x| traverse(x, dims, Direction::Left))
        }

        Direction::Left => (column > 0).then(|| idx - 1),

        Direction::UpLeft => {
            traverse(idx, dims, Direction::Up).and_then(|x| traverse(x, dims, Direction::Left))
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count();
    let search_string: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_ascii())
        .collect();
    let width = search_string.len() / height;
    let dims = GridDimensions { width, height };
    let search_bytes = search_string.as_bytes();

    let count = Direction::iter().fold(0, |acc, dir| {
        let mut count = acc;
        for idx in 0..search_bytes.len() {
            let first_idx = idx;
            if search_bytes[first_idx] != b'X' {
                continue;
            }
            let Some(second_idx) = traverse(first_idx, dims, dir) else {
                continue;
            };
            if search_bytes[second_idx] != b'M' {
                continue;
            }
            let Some(third_idx) = traverse(second_idx, dims, dir) else {
                continue;
            };
            if search_bytes[third_idx] != b'A' {
                continue;
            }
            let Some(fourth_idx) = traverse(third_idx, dims, dir) else {
                continue;
            };
            if search_bytes[fourth_idx] != b'S' {
                continue;
            }
            count += 1;
        }
        count
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
