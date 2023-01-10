use crate::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Roll {
    Pins(u16),
    Spare,
    Strike,
}

impl From<Roll> for u16 {
    fn from(roll: Roll) -> Self {
        match roll {
            Roll::Pins(pins) => pins,
            _ => 10,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct Frame {
    pub rolls: Vec<Roll>,
    pub state: FrameState,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            rolls: vec![],
            state: FrameState::Open,
        }
    }
}

impl Frame {
    pub fn add_roll(&mut self, pins: u16) -> Result<(), Error> {
        if let Some(Roll::Pins(first_roll_pins)) = self.rolls.get(0) {
            match first_roll_pins + pins {
                0..=9 => self.rolls.push(Roll::Pins(pins)),
                10 => self.rolls.push(Roll::Spare),
                _ => return Err(Error::NotEnoughPinsLeft),
            }
            self.state = FrameState::Closed;
        } else {
            match pins {
                0..=9 => self.rolls.push(Roll::Pins(pins)),
                10 => {
                    self.rolls.push(Roll::Strike);
                    self.state = FrameState::Closed;
                }
                _ => return Err(Error::NotEnoughPinsLeft),
            }
        }
        Ok(())
    }
}
