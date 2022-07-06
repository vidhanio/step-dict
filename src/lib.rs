#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]
#![feature(step_trait)]

//! # step through dictionary words
//!
//! ## Usage
//!
//! ```
//! use step_dict::Word;
//!
//! for word in Word("rust")..Word("rusty") {
//!     println!("{}", word);
//! }
//! ```

use std::{
    fmt::{self, Display, Formatter},
    iter::Step,
};

const WORDS: &[&str] = &include!("words_alpha.txt");

/// A word in the dictionary.
///
/// # Examples
///
/// ```
/// use step_dict::Word;
///
/// for word in Word("rust")..Word("rusty") {
///     println!("{}", word);
/// }
/// ```
///
/// # Panics
///
/// This will panic if the word is not in the dictionary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word<'a>(pub &'a str);

impl Step for Word<'_> {
    fn steps_between(&Self(start): &Self, &Self(end): &Self) -> Option<usize> {
        let start = WORDS
            .binary_search(&start)
            .expect("start should be in WORDS");
        let end = WORDS.binary_search(&end).expect("end should be in WORDS");
        Some(end - start)
    }

    fn forward_checked(Self(start): Self, count: usize) -> Option<Self> {
        let start = WORDS
            .binary_search(&start)
            .expect("start, should be in WORDS");
        let end = start + count;

        WORDS.get(end).map(|s| Self(s))
    }

    fn backward_checked(Self(start): Self, count: usize) -> Option<Self> {
        let start = WORDS
            .binary_search(&start)
            .expect("start should be in WORDS");
        let end = start - count;

        WORDS.get(end).map(|s| Self(s))
    }
}

impl Display for Word<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}