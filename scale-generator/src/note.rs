use std::{collections::HashMap, fmt::Display};

use crate::{failure::Error, Result, CHROMATIC_SCALE, FLAT_POST_FIX};

#[derive(Debug, PartialEq, Eq)]
pub enum NoteType {
    Flat,
    Sharp,
    Regular,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum NoteCategory {
    Flats,
    Sharps,
    None,
}

fn is_flat_note(note: &str) -> bool {
    note.contains('b')
}

pub fn into_flat(note: &str) -> Result<String> {
    let note = CHROMATIC_SCALE
        .iter()
        .map(|&n| Note::new(n))
        .cycle()
        .skip_while(|n| n.inner.ne(&note))
        .nth(1)
        .ok_or(Error::new(
            "There should always be notes of the chromatic scale",
        ))?;
    Ok(format!("{}{}", note, FLAT_POST_FIX))
}

pub fn into_sharp(note: &str) -> Result<String> {
    let index = CHROMATIC_SCALE
        .iter()
        .map(|&n| Note::new(n))
        .position(|n| {
            let flat_char = note.chars().nth(0).unwrap();
            n.inner.eq(&flat_char.to_uppercase().to_string())
        })
        .ok_or(Error::new("Not a valid note."))?;

    let note = CHROMATIC_SCALE
        .iter()
        .nth(index - 1)
        .ok_or(Error::new(&format!(
            "Cannot retrieve note at inded {}.",
            index - 1
        )))?;

    Ok(note.to_string())
}

fn is_sharp_note(note: &str) -> bool {
    note.contains('#')
}

fn get_note_type(note: &str) -> NoteType {
    match note {
        n if is_flat_note(n) => NoteType::Flat,
        n if is_sharp_note(n) => NoteType::Sharp,
        _ => NoteType::Regular,
    }
}

fn get_note_category(note: &str) -> Result<NoteCategory> {
    let mut category_map = HashMap::new();
    category_map.insert(
        NoteCategory::Flats,
        vec![
            "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
        ],
    );
    category_map.insert(NoteCategory::None, vec!["C", "a"]);
    category_map.insert(
        NoteCategory::Sharps,
        vec![
            "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", " d#",
        ],
    );
    let category = category_map
        .into_iter()
        .find(|(_cat, notes)| notes.iter().any(|&n| n.eq(note)))
        .map(|(cat, _)| cat)
        .ok_or(Error::new("Unable to determine category for note."))?;
    Ok(category)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Note {
    pub inner: String,
    pub category: NoteCategory,
    pub note_type: NoteType,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Note {
    pub fn new(inner: &str) -> Self {
        Self {
            inner: inner.to_owned(),
            category: get_note_category(inner).unwrap_or(NoteCategory::None),
            note_type: get_note_type(inner),
        }
    }
}
