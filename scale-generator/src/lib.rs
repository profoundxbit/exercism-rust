#![allow(dead_code)]

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

mod failure;

use std::vec;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;
type Sharps = 

enum NoteType {
    Sharp {
        major: [&str; 6],
        minor: [&str; 6]
    },
    Flats {
        major: [&str; 6],
        minor:  [&str; 6]
    }
}



fn use_sharps(tonic :&str) -> bool {
    let sharps = NoteType::Sharp { major: ["G", "D", "A", "E", "B", "F#"], minor: ["e", "b", "f#", "c#", "g#", "d#"] };
    if tonic.chars().nth(0).unwrap().is_uppercase() {
        
    }
}

const CHROMATIC_SCALE: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLATS: [&str; 6] = ["F", "Bb", "Eb", "Ab", "Db", "Gb"];

const FLAT_POST_FIX: &str = "b";

fn sharp_to_flat(sharp_note: &str) -> Result<String> {
    if !sharp_note.contains('#') {
        return Err(Error::new("Sharp notes must contain '#' character"));
    }

    let index = CHROMATIC_SCALE
        .iter()
        .position(|&x| x == sharp_note)
        .ok_or(Error::new(&format!(
            "Note {} does not exist in Chromatic scale",
            sharp_note
        )))?;

    // What if last element in array? 'G#'
    let next_note: &str;
    if index == 11 {
        next_note = CHROMATIC_SCALE.get(0).ok_or(Error::new(
            "Unable to get first character of Chromatic scale",
        ))?;
    } else {
        next_note = CHROMATIC_SCALE.get(index + 1).ok_or(Error::new(&format!(
            "Unable to get character at index {} from Chromatic scale",
            index
        )))?;
    }
    let mut next_note = next_note.to_owned();
    next_note.push_str(&FLAT_POST_FIX);
    Ok(next_note)
}

pub struct Scale {
    notes: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale> {
        
        match
        
        Ok(Self {
            notes: Default::default(),
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale> {
        let index = CHROMATIC_SCALE
            .iter()
            .position(|&x| x == tonic)
            .ok_or(Error::new("Incorrect tonic provided"))?;
        let mut notes = CHROMATIC_SCALE
            .iter()
            .cycle()
            .skip(index + 1)
            .take_while(|&x| *x != tonic)
            .map(|y| y.to_string())
            .collect::<Vec<String>>();
        notes.push(tonic.to_owned());
        let mut tonic_vec = vec![tonic.to_owned()];
        tonic_vec.append(&mut notes);
        let mut notes = tonic_vec;

        if FLATS.contains(&tonic) {
            for note in notes.iter_mut() {
                if note.contains('#') {
                    dbg!(&note);
                    *note = sharp_to_flat(note)?.to_string();
                }
            }
        }
        Ok(Scale { notes })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
