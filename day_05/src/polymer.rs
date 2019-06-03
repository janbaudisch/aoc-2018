#[derive(Clone)]
pub struct Polymer(Vec<char>);

impl Polymer {
    // check wether the current element and the next react
    fn check(a: char, b: char) -> bool {
        // explanation: every unicode letter (a-z) is exactly 32 code points apart from its lowercase/uppercase counterpart
        (a as i32 - b as i32).abs() == 32
    }

    // fully react the polymer
    pub fn react(&mut self) {
        let mut i: usize = 0;

        // run through whole polymer
        while i + 2 < self.0.len() {
            // check current and following element
            if Self::check(self.0[i], self.0[i + 1]) {
                // if reacting, remove next ...
                self.0.remove(i + 1);
                // ... and current element
                self.0.remove(i);

                // step back once to check for collapsed polymer
                if i > 0 {
                    i -= 1;
                }
            } else {
                // step forward
                i += 1;
            }
        }
    }

    // filter out one element (not case sensitive)
    pub fn filter(&mut self, filter: char) {
        self.0
            .retain(|&x| x.to_lowercase().next().unwrap() != filter);
    }

    // count elements in polymer
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// convert string to polymer
impl From<String> for Polymer {
    fn from(string: String) -> Self {
        let mut chars = Vec::new();
        chars.append(&mut string.chars().collect::<Vec<char>>());
        Polymer(chars)
    }
}

// access iterator over elements
impl IntoIterator for Polymer {
    type Item = char;
    type IntoIter = ::std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
