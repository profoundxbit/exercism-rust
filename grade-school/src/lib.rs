use std::collections::BTreeMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School<'a> {
    roster: BTreeMap<u32, Vec<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            roster: Default::default(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.roster
            .entry(grade)
            .and_modify(|g| g.push(student))
            .or_insert_with(|| vec![student]);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = match self.roster.get(&grade) {
            Some(students) => students.to_owned(),
            None => Default::default(),
        };
        students.sort();
        students.iter().map(|x| x.to_string()).collect()
    }
}
