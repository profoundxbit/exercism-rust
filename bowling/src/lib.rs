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
            } // Empty frame so either push strike or pins
            1 => {
                if let Some(Roll::Pins(x)) = frame.rolls.get(0) {
                    if x + pins == 10 {
                        frame.rolls.push(Roll::Spare);
                    } else {
                        if pins == 10 {
                            frame.rolls.push(Roll::Strike);
                        } else {
                            frame.rolls.push(Roll::Pins(pins));
                            frame.state = FrameState::Closed;
                        }
                    }
                }
            } // Rolled once so either push spare or pins or strike.
            2 => {
                if pins == 10 {
                    frame.rolls.push(Roll::Strike);
                } else {
                    frame.rolls.push(Roll::Pins(pins));
                    frame.state = FrameState::Closed;
                }
            } //Rolled twice so its either last roll was spare or strike. Push pins or strike
            _ => return Err(Error::GameComplete),
        };

        Ok(())
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
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

        dbg!(self);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        Some(0)
    }
}
