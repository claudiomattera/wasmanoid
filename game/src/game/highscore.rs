// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "storage")]
use serde::{Deserialize, Serialize};

use tinyvec::array_vec;
use tinyvec::ArrayVec;

use wasm4fun_log::debug;
use wasm4fun_storage::{load, store};

const MAX_HIGH_SCORES: usize = 5;
const MAX_HIGH_SCORES_PLUS_ONE: usize = MAX_HIGH_SCORES + 1;

/// A highscore
#[derive(Debug)]
#[cfg_attr(feature = "storage", derive(Deserialize, Serialize))]
pub struct HighScore {
    name: [u8; 3],
    score: u32,
}

impl HighScore {
    /// Create a new highscore
    pub fn new(name: [char; 3], score: u32) -> Self {
        let name = [name[0] as u8, name[1] as u8, name[2] as u8];
        Self { name, score }
    }

    /// Return the name associated to the highscore
    pub fn name(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.name) }
    }

    /// Return the score associated to the highscore
    pub fn score(&self) -> u32 {
        self.score
    }
}

impl Default for HighScore {
    fn default() -> Self {
        Self::new(['A', 'A', 'A'], 0)
    }
}

/// A list of highscores
#[derive(Debug)]
#[cfg_attr(feature = "storage", derive(Deserialize, Serialize))]
pub struct HighScores(ArrayVec<[HighScore; MAX_HIGH_SCORES_PLUS_ONE]>);

impl HighScores {
    /// Check whether the argument should be added to the highscore
    ///
    /// This function returns true if either a) the highscore list is not
    /// complete yet, or b) if the argument has a better score than any of the
    /// ones in the list.
    pub fn is_beated_by(&self, other_score: u32) -> bool {
        self.0.len() < MAX_HIGH_SCORES
            || self
                .0
                .iter()
                .any(|highscore| other_score > highscore.score())
    }

    /// Add a highscore to the list, possibly removing a worse one
    pub fn add(&mut self, highscore: HighScore) {
        self.0.push(highscore);
        self.0.sort_unstable_by(|highscore1, highscore2| {
            highscore1.score.partial_cmp(&highscore2.score).unwrap()
        });
        self.0.reverse();
        self.0.truncate(MAX_HIGH_SCORES);
    }

    /// Save the highscores to storage
    pub fn save(&self) {
        store::<&Self, 128>(self)
    }

    /// Save the highscores from storage
    pub fn load() -> Self {
        let highscores: Self = load::<Self, 128>();
        debug!("Loaded {} highscores", highscores.len());
        highscores
    }

    /// Return an iterator over the highscores
    pub fn iter(&self) -> impl Iterator<Item = &HighScore> {
        self.0.iter()
    }

    /// Checks whether the list is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return the length of the list
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Default for HighScores {
    fn default() -> Self {
        Self(array_vec!())
    }
}
