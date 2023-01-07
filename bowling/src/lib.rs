mod frame;
use frame::{Frame, FrameState};
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

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let current_frame = self
            .scoreboard
            .iter_mut()
            .find(|frame| frame.state == FrameState::Open)
            .ok_or(Error::GameComplete)?;

        let _ = current_frame.add_roll(pins)?;
        dbg!(self);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        Some(0)
    }
}
