
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
mod note;

use failure::Error;
use note::{into_flat, into_sharp, Note, NoteCategory, NoteType};

pub type Result<T> = std::result::Result<T, Error>;

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
        // Produce chromatic scale based on tonic and interval
        // Modify scale based on tonic category

        let tonic = Note::new(tonic);
        let index = CHROMATIC_SCALE
            .iter()
            .position(|&n| {
                if tonic.note_type == NoteType::Flat {
                    let tonic_note = into_sharp(&tonic.inner).unwrap();
                    return n.eq(&tonic_note);
                }

                n.eq(&tonic.inner.to_uppercase())
            })
            .ok_or(Error::new("Unable to locate tonic"))?;
        let mut note_cycle = CHROMATIC_SCALE.iter().map(|&n| Note::new(n)).cycle();

        let tonic_note = note_cycle
            .nth(index)
            .ok_or(Error::new("Unable to retrieve tonic note"))?;

        let mut notes = vec![tonic_note];

        for interval in intervals.chars() {
            match interval {
                'm' => {
                    let note = note_cycle
                        .next()
                        .ok_or(Error::new("Unable to retrieve next note"))?;
                    notes.push(note);
                }
                'M' => {
                    let note = note_cycle
                        .nth(1)
                        .ok_or(Error::new("Unable to retrieve next note"))?;
                    notes.push(note);
                }
                'A' => {
                    let note = note_cycle
                        .nth(2)
                        .ok_or(Error::new("Unable to retrieve next note"))?;
                    notes.push(note);
                }
                _ => return Err(Error::new(&format!("Invalid interval: {}", &interval))),
            }
        }
        let notes = match tonic.category {
            NoteCategory::None => notes
                .iter()
                .map(|x| x.inner.to_uppercase())
                .collect::<Vec<_>>(),
            NoteCategory::Sharps => notes
                .into_iter()
                .map(|x| match x.note_type.eq(&NoteType::Flat) {
                    true => into_sharp(&x.inner).unwrap(),
                    false => x.inner.to_owned(),
                })
                .collect::<Vec<_>>(),
            NoteCategory::Flats => notes
                .into_iter()
                .map(|x| match x.note_type.eq(&NoteType::Sharp) {
                    true => into_flat(&x.inner).unwrap(),
                    false => x.inner.to_owned(),
                })
                .collect::<Vec<_>>(),
        };

        Ok(Self { notes })
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
