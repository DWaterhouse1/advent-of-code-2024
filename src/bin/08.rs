use grid::Grid;

advent_of_code::solution!(8);

#[derive(Clone, PartialEq, Eq)]
enum Antinode {
    Present,
    Absent,
}

#[derive(Clone, PartialEq, Eq)]
struct GridCoord {
    row: usize,
    col: usize,
}

fn grid_coord_from_index(idx: usize, width: usize) -> GridCoord {
    GridCoord {
        row: idx / width,
        col: idx % width,
    }
}

const NUM_ALPHANUMERICS: usize = 62;

fn alphanumeric_to_index(ch: char) -> Option<u8> {
    let value = u8::try_from(ch).ok()?;
    match value as u8 {
        b'0'..=b'9' => Some(value - b'0'),
        b'A'..=b'Z' => Some(value - (b'A' - 10)),
        b'a'..=b'z' => Some(value - (b'a' - 36)),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count();
    let striped: String = input
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_ascii())
        .collect();
    let width = striped.len() / height;

    let mut map = Grid::init(height, width, Antinode::Absent);

    let mut positions_by_frequency = vec![<Vec<GridCoord>>::new(); NUM_ALPHANUMERICS];

    for (idx, ch) in striped.chars().enumerate() {
        if let Some(station_freq) = alphanumeric_to_index(ch) {
            positions_by_frequency
                .get_mut(station_freq as usize)?
                .push(grid_coord_from_index(idx, width));
        }
    }

    for frequency_set in positions_by_frequency {
        for (idx, station_one) in frequency_set.iter().enumerate() {
            for station_two in frequency_set.iter().skip(idx + 1) {
                let row_dif = station_one.row as i64 - station_two.row as i64;
                let col_dif = station_one.col as i64 - station_two.col as i64;

                let an_one_row = station_one.row as i64 + row_dif;
                let an_one_col = station_one.col as i64 + col_dif;

                let an_two_row = station_two.row as i64 - row_dif;
                let an_two_col = station_two.col as i64 - col_dif;

                if an_one_row >= 0 && an_one_col >= 0 {
                    if let Some(tile) = map.get_mut(an_one_row, an_one_col) {
                        *tile = Antinode::Present;
                    }
                }

                if an_two_row >= 0 && an_two_col >= 0 {
                    if let Some(tile) = map.get_mut(an_two_row, an_two_col) {
                        *tile = Antinode::Present;
                    }
                }
            }
        }
    }

    let antinode_count = map.iter().filter(|x| **x == Antinode::Present).count();

    u32::try_from(antinode_count).ok()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
