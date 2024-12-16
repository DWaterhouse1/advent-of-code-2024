advent_of_code::solution!(4);

use strum::IntoEnumIterator;

use advent_of_code::helpers::grids::*;

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

fn is_m_or_s(byte: u8) -> bool {
    byte == b'M' || byte == b'S'
}

fn test_direction_pair(
    first: Direction,
    second: Direction,
    idx: usize,
    dims: GridDimensions,
    bytes: &[u8],
) -> bool {
    let Some(first_value) = traverse(idx, dims, first)
        .map(|x| bytes[x])
        .filter(|byte| is_m_or_s(*byte))
    else {
        return false;
    };

    let Some(second_value) = traverse(idx, dims, second)
        .map(|x| bytes[x])
        .filter(|byte| is_m_or_s(*byte))
    else {
        return false;
    };

    first_value != second_value
}

pub fn part_two(input: &str) -> Option<u32> {
    let height = input.lines().count();
    let search_string: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_ascii())
        .collect();
    let width = search_string.len() / height;
    let dims = GridDimensions { width, height };
    let search_bytes = search_string.as_bytes();

    let mut count = 0;
    for idx in 0..search_bytes.len() {
        if search_bytes[idx] != b'A' {
            continue;
        }

        if test_direction_pair(
            Direction::UpRight,
            Direction::DownLeft,
            idx,
            dims,
            search_bytes,
        ) && test_direction_pair(
            Direction::UpLeft,
            Direction::DownRight,
            idx,
            dims,
            search_bytes,
        ) {
            count += 1;
        }
    }

    Some(count)
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
        assert_eq!(result, Some(9));
    }
}
