use advent_of_code::helpers::grids::{traverse, Direction, GridDimensions};

advent_of_code::solution!(6);

// TODO cleanup this horrible code. It "works", but gosh is it ever ugly

#[derive(Clone, Copy)]
enum Tile {
    Start,
    Unvisited,
    Visited(u8),
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
            Tile::Unvisited => Tile::Visited(0),
            Tile::Visited(n) => Tile::Visited(*n + 1),
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

    while let Some(next_guard_pos) = traverse(guard_position, dims, guard_facing) {
        match map[next_guard_pos] {
            Tile::Unvisited => {
                num_visited += 1;
                map[next_guard_pos].visit();
                guard_position = next_guard_pos;
            }
            Tile::Visited(_) => {
                map[next_guard_pos].visit();
                guard_position = next_guard_pos;
            }
            Tile::Wall => {
                guard_facing.turn_right();
            }
            Tile::Start => {
                match guard_facing {
                    Direction::Up => {
                        // oh dear, we're in a loop
                        break;
                    }
                    _ => {
                        // normal visit
                        map[next_guard_pos].visit();
                        guard_position = next_guard_pos;
                    }
                }
            }
        }
    }

    Some(num_visited)
}

pub fn part_two(input: &str) -> Option<u32> {
    let height = input.lines().count();

    let mut maybe_guard_idx: Option<usize> = None;
    let map: Vec<Tile> = input
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

    let mut candidates = Vec::<usize>::new();
    {
        let mut guard_facing = Direction::Up;
        let mut guard_position = maybe_guard_idx?;

        let mut candidates_trial_map = map.clone();

        while let Some(next_guard_pos) = traverse(guard_position, dims, guard_facing) {
            match candidates_trial_map[next_guard_pos] {
                Tile::Unvisited => {
                    candidates.push(next_guard_pos);
                    candidates_trial_map[next_guard_pos].visit();
                    guard_position = next_guard_pos;
                }
                Tile::Visited(_) => {
                    candidates_trial_map[next_guard_pos].visit();
                    guard_position = next_guard_pos;
                }
                Tile::Wall => {
                    guard_facing.turn_right();
                }
                Tile::Start => {
                    match guard_facing {
                        Direction::Up => {
                            // oh dear, we're in a loop
                            break;
                        }
                        _ => {
                            // normal visit
                            candidates_trial_map[next_guard_pos].visit();
                            guard_position = next_guard_pos;
                        }
                    }
                }
            }
        }
    }

    let mut num_loops: u32 = 0;

    for candidate_wall in candidates.iter() {
        let mut guard_facing = Direction::Up;
        let mut guard_position = maybe_guard_idx?;
        let mut trial_map = map.clone();
        trial_map[*candidate_wall] = Tile::Wall;
        trial_map[guard_position] = Tile::Visited(1);

        let mut found_loop = false;

        while let Some(next_guard_pos) = traverse(guard_position, dims, guard_facing) {
            match trial_map[next_guard_pos] {
                Tile::Wall => {
                    guard_facing.turn_right();
                }
                Tile::Visited(num_visits) => {
                    if num_visits == 3 {
                        found_loop = true;
                        break;
                    }
                    trial_map[next_guard_pos].visit();
                    guard_position = next_guard_pos;
                }
                Tile::Unvisited => {
                    trial_map[next_guard_pos].visit();
                    guard_position = next_guard_pos;
                }
                Tile::Start => {
                    panic!("We already removed this one!");
                }
            }
        }

        if found_loop {
            num_loops += 1;
        }
    }

    Some(num_loops)
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
        assert_eq!(result, Some(6));
    }
}
