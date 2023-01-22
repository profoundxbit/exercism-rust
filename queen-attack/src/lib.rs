use std::ops::AddAssign;

#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy)]
struct Diagonal(Direction, Direction);

impl AddAssign<Direction> for i32 {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => *self += -1,
            Direction::Left => *self += -1,
            Direction::Down => *self += 1,
            Direction::Right => *self += 1,
        }
    }
}

impl ChessPosition {
    fn invalid_position(position: i32) -> bool {
        position.is_negative() || position > 7
    }

    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if Self::invalid_position(rank) || Self::invalid_position(file) {
            None
        } else {
            Some(Self { rank, file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    fn can_attack_diagonal(&self, other_position: &ChessPosition) -> bool {
        let ChessPosition { mut rank, mut file } = self.position;
        let ChessPosition {
            rank: other_rank,
            file: other_file,
        } = other_position;
        let diagonal_direction = if rank > *other_rank {
            if file > *other_file {
                Diagonal(Direction::Up, Direction::Left)
            } else {
                Diagonal(Direction::Up, Direction::Right)
            }
        } else {
            if file > *other_file {
                Diagonal(Direction::Down, Direction::Left)
            } else {
                Diagonal(Direction::Down, Direction::Right)
            }
        };

        while !ChessPosition::invalid_position(rank) && !ChessPosition::invalid_position(file) {
            rank += diagonal_direction.0;
            file += diagonal_direction.1;

            if (rank, file) == (*other_rank, *other_file) {
                return true;
            }
        }
        false
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        other.position.rank == self.position.rank
            || other.position.file == self.position.file
            || self.can_attack_diagonal(&other.position)
    }
}
