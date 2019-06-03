use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct ID(Vec<char>);

impl ID {
    pub fn scan(&self) -> (bool, bool) {
        let mut duplicates = HashMap::new();

        for character in &self.0 {
            duplicates
                .entry(character)
                .or_insert_with(Vec::new)
                .push(character);
        }

        let mut two = false;
        let mut three = false;

        for value in duplicates.values() {
            if value.len() == 2 {
                two = true;
            }

            if value.len() == 3 {
                three = true;
            }
        }

        (two, three)
    }

    pub fn diff(&self, other: &ID) -> Vec<usize> {
        let mut diff = Vec::new();

        for (i, character) in self.0.clone().into_iter().enumerate() {
            if character != other.0[i] {
                diff.push(i);
            }
        }

        diff
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

impl From<String> for ID {
    fn from(string: String) -> Self {
        let mut characters = Vec::new();
        characters.append(&mut string.chars().collect::<Vec<char>>());

        ID(characters)
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for character in &self.0 {
            write!(formatter, "{}", character)?;
        }

        Ok(())
    }
}
