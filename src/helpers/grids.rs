use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy)]
pub struct GridDimensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, EnumIter, Display)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,

            // not actually necessary for us, but better than a panic
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpLeft,
            Direction::UpLeft => Direction::UpRight,
        }
    }
}

pub fn traverse(idx: usize, dims: GridDimensions, direction: Direction) -> Option<usize> {
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
