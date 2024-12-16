use advent_of_code::helpers::grids::{traverse, Direction, GridDimensions};

advent_of_code::solution!(6);

enum Tile {
    Start,
    Unvisited,
    Visited,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Unvisited),
            '#' => Ok(Tile::Wall),
            '^' => Ok(Tile::Start),
            _ => Err(()),
        }
    }
}

impl Tile {
    fn visit(&mut self) {
        *self = match self {
            Tile::Start => Tile::Start,
            Tile::Unvisited => Tile::Visited,
            Tile::Visited => Tile::Visited,
            Tile::Wall => panic!("We really shouldn't be visiting a wall."),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let height = input.lines().count();

    let mut maybe_guard_idx: Option<usize> = None;
    let mut map: Vec<Tile> = input
        .chars()
        .filter(|c| !c.is_whitespace() && c.is_ascii())
        .enumerate()
        .map(|(idx, ch)| {
            if ch == '^' {
                maybe_guard_idx = Some(idx)
            }
            TryFrom::try_from(ch).ok()
        })
        .collect::<Option<Vec<Tile>>>()?;

    let width = map.len() / height;
    let dims = GridDimensions { width, height };

    let mut num_visited = 1; // the guard has visited the tile she started on
    let mut guard_facing = Direction::Up;
    let mut guard_position = maybe_guard_idx?;

    loop {
        let next = match traverse(guard_position, dims, guard_facing) {
            Some(next_guard_pos) => match map[next_guard_pos] {
                Tile::Unvisited => {
                    num_visited += 1;
                    next_guard_pos
                }
                Tile::Visited => next_guard_pos,
                Tile::Wall => {
                    guard_facing.turn_right();
                    guard_position
                }
                Tile::Start => {
                    match guard_facing {
                        Direction::Up => {
                            // oh dear, we're in a loop
                            break;
                        }
                        _ => {
                            // normal visit
                            next_guard_pos
                        }
                    }
                }
            },
            None => {
                break;
            }
        };
        guard_position = next;
        map[next].visit();
    }

    Some(num_visited)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
