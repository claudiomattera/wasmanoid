// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm4fun_fmt::format_i32;
use wasm4fun_graphics::{
    draw_horizontal_line, draw_rect, draw_text, set_drawing_colors, SCREEN_SIZE,
};
use wasm4fun_input::GamePad;

use crate::audio::{menu_move_tone, menu_select_tone, AudioQueue};
use crate::game::{HighScore, HighScores};
use crate::graphics::draw_centered_5x8_text;

use super::Transition;

const MAX_COOLDOWN: u8 = 10;
const MAX_LETTERS: usize = 3;

pub struct SaveScoreState {
    cooldown: u8,
    index: usize,
    letters: [char; MAX_LETTERS],
    score: u32,
}

impl SaveScoreState {
    pub fn new(score: u32) -> Self {
        Self {
            cooldown: MAX_COOLDOWN,
            index: 0,
            letters: ['A'; MAX_LETTERS],
            score,
        }
    }

    pub fn draw(&self, _gamepad: Option<&GamePad>) {
        set_drawing_colors(4);
        draw_rect(0, 0, SCREEN_SIZE, SCREEN_SIZE);

        set_drawing_colors(1);

        let text = "Achieved high score!";
        let y = ((SCREEN_SIZE - 72) / 2) as i32;
        draw_centered_5x8_text(text, y);

        let mut buffer = [0; 10];
        let text = format_i32(&mut buffer, self.score as i32);
        let y = ((SCREEN_SIZE - 48) / 2) as i32;
        draw_centered_5x8_text(text, y);

        let text = "Enter name:";
        let y = ((SCREEN_SIZE - 16) / 2) as i32;
        draw_centered_5x8_text(text, y);

        let x = ((SCREEN_SIZE - 5 * 8) / 2) as i32;
        let y = ((SCREEN_SIZE + 16) / 2) as i32;

        let mut tmp = [0u8; 1];

        for (i, letter) in self.letters.iter().enumerate() {
            let s = letter.encode_utf8(&mut tmp);
            draw_text(s, x + 16 * i as i32, y);
        }

        draw_horizontal_line(x + 16 * self.index as i32 - 2, y + 8, 11);
        draw_horizontal_line(x + 16 * self.index as i32 - 2, y + 9, 11);
    }

    pub fn update(&mut self, gamepad: &GamePad, audio_queue: &mut AudioQueue) -> Transition {
        if self.cooldown == 0 {
            if gamepad.z() {
                let mut highscores = HighScores::load();
                let highscore = HighScore::new(self.letters, self.score);
                highscores.add(highscore);
                highscores.save();
                audio_queue.play(menu_select_tone());

                return Transition::PopN(2);
            } else if gamepad.down() {
                let mut letter = self.letters[self.index] as u8;
                letter += 1;
                if letter > b'Z' {
                    letter = b'A';
                }
                self.letters[self.index] = letter as char;
                audio_queue.play(menu_move_tone());
                self.cooldown = MAX_COOLDOWN;
            } else if gamepad.up() {
                let mut letter = self.letters[self.index] as u8;
                letter -= 1;
                if letter < b'A' {
                    letter = b'Z';
                }
                self.letters[self.index] = letter as char;
                audio_queue.play(menu_move_tone());
                self.cooldown = MAX_COOLDOWN;
            } else if gamepad.left() {
                if self.index > 0 {
                    self.index -= 1;
                }
                audio_queue.play(menu_move_tone());
                self.cooldown = MAX_COOLDOWN;
            } else if gamepad.right() {
                if self.index < MAX_LETTERS - 1 {
                    self.index += 1;
                }
                audio_queue.play(menu_move_tone());
                self.cooldown = MAX_COOLDOWN;
            }
        }

        if self.cooldown > 0 {
            self.cooldown -= 1;
        }

        Transition::Noop
    }
}
