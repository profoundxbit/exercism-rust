mod frame;
use frame::{Frame, FrameState};

use crate::frame::Roll;
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    scoreboard: [Frame; 10],
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            scoreboard: Default::default(),
        }
    }

    fn handle_last_frame(frame: &mut Frame, pins: u16) -> Result<(), Error> {
        match frame.rolls.len() {
            0 => {
                if pins == 10 {
                    frame.rolls.push(Roll::Strike);
                } else {
                    frame.rolls.push(Roll::Pins(pins));
                }
            }
            1 => {
                if let Some(Roll::Pins(x)) = frame.rolls.get(0) {
                    if x + pins == 10 {
                        frame.rolls.push(Roll::Spare);
                    } else {
                        frame.rolls.push(Roll::Pins(pins));
                        frame.state = FrameState::Closed;
                    }
                } else {
                    if pins == 10 {
                        frame.rolls.push(Roll::Strike);
                    } else {
                        frame.rolls.push(Roll::Pins(pins));
                    }
                }
            }
            2 => {
                match frame.rolls.get(1) {
                    Some(Roll::Pins(second_throw_pins)) => {
                        if second_throw_pins + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        frame.rolls.push(Roll::Pins(pins));
                    }
                    Some(Roll::Spare) | Some(Roll::Strike) => {
                        if pins == 10 {
                            frame.rolls.push(Roll::Strike)
                        } else {
                            frame.rolls.push(Roll::Pins(pins))
                        }
                    }
                    _ => panic!("Invalid second roll of last frame"),
                }
                frame.state = FrameState::Closed;
            } 
            _ => return Err(Error::GameComplete),
        };
        Ok(())
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        let (current_frame_index, mut current_frame) = self
            .scoreboard
            .iter_mut()
            .enumerate()
            .find(|(_index, frame)| frame.state == FrameState::Open)
            .ok_or(Error::GameComplete)?;

        if current_frame_index == 9 {
            let _ = BowlingGame::handle_last_frame(&mut current_frame, pins)?;
        } else {
            let _ = current_frame.add_roll(pins)?;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self
            .scoreboard
            .iter()
            .any(|frame| frame.state == FrameState::Open)
        {
            return None;
        }

        let score = self
            .scoreboard
            .iter()
            .enumerate()
            .fold(0u16, |mut total, (index, frame)| {
                if index == 9 {
                    match frame.rolls[..] {
                        [Roll::Strike, Roll::Strike, Roll::Strike] => total += 30,
                        [Roll::Strike, _, Roll::Spare] | [_, Roll::Spare, Roll::Strike] => total +=20,
                        [Roll::Strike, Roll::Pins(x), Roll::Pins(y)] => total += 10 + x + y,
                        [_, Roll::Spare, Roll::Pins(pins)] => total += 10 + pins,
                        [Roll::Pins(first_throw_pins), Roll::Pins(second_row_pins)] => total += first_throw_pins + second_row_pins,
                        _ => panic!("Invalid final frame {:?}", frame)
                    };
                } else {
                match frame.rolls[..] {
                    [Roll::Strike] => {
                        total += 10;
                        let next_frame = &self.scoreboard[index + 1];
                        let next_frame_first_roll = &next_frame.rolls[0];
                        if *next_frame_first_roll == Roll::Strike {
                            total += 10;
                            if index == &self.scoreboard.len() - 2 {
                                match next_frame.rolls[..] {
                                    [_, Roll::Strike, _] => total += 10,
                                    [_, Roll::Pins(second_roll_pins), _] => total += second_roll_pins,
                                    _ => panic!("Invalid last frame")
                                }
                            } else {
                            match &self.scoreboard[index + 2].rolls[0] {
                                Roll::Strike => total += 10,
                                Roll::Pins(pins) => total += pins,
                                _ => panic!("First roll of a frame cannot be a spare."),
                            }
                        }
                        } else {
                            match next_frame.rolls[..] {
                                [_, Roll::Spare] => total += 10,
                                [Roll::Pins(first_roll_pins), Roll::Pins(second_roll_pins)] => total += first_roll_pins + second_roll_pins,
                                _ => panic!("Normal frame can only be either a set of pins, spare, or strike.")
                            }
                        }
                    }
                    [Roll::Pins(first_roll_pins), Roll::Pins(second_roll_pins)] => {
                        total += first_roll_pins + second_roll_pins;
                    },
                    [_, Roll::Spare] => {
                        total += 10;
                        let next_frame = &self.scoreboard[index+1];
                        let next_frame_first_roll = &next_frame.rolls[0];
                        match *next_frame_first_roll {
                                Roll::Strike => total += 10,
                                Roll::Pins(pins) => total += pins,
                                _ => panic!("First roll of a frame cannot be a spare."),
                        };
                    },
                    _ => panic!("Invalid frame {:?}", &frame),
                };
            }
                total
            });
        Some(score)
    }
}
