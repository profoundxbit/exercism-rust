#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
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

    pub fn can_attack(&self, other: &Queen) -> bool {
        match other.position {
            ref o if o.rank == self.position.rank || o.file == self.position.file => true,
            _ => false
        }
    }
}
