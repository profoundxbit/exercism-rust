// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl From<i8> for Direction {
    fn from(value: i8) -> Self {
        match value {
            0 | 4 => Self::North, // Handles turning right when currently facing West
            1 => Self::East,
            2 => Self::South,
            -1 | 3 => Self::West, // Handles turning left when currently facing North
            _ => unreachable!(
                "Not a valid direction. Valid directions are North, East, South, and West."
            ),
        }
    }
}

#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let mut direction = self.direction as i8;
        direction += 1;

        Self::new(self.x, self.y, direction.into())
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let mut direction = self.direction as i8;
        direction -= 1;

        Self::new(self.x, self.y, direction.into())
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut robot = self;
        use crate::Direction::*;
        match robot.direction {
            North => robot.y += 1,
            East => robot.x += 1,
            South => robot.y -= 1,
            West => robot.x -= 1,
            _ => unreachable!("Unable to advance. Not a valid direction. Valid directions are North, East, South, and West.")
        };
        robot
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        let mut instructions = instructions.chars();
        let mut instruction = instructions.next();
        while let Some(movement) = instruction {
            match movement {
                'R' => {
                    robot = robot.turn_right();
                }
                'L' => {
                    robot = robot.turn_left();
                }
                'A' => {
                    robot = robot.advance();
                }
                _ => unreachable!(
                    "Robots only have three possible movements; turn right, turn left, or advance."
                ),
            };
            instruction = instructions.next();
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
