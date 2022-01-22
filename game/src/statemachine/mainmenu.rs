// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm4fun_fmt::format_i32_padded;
use wasm4fun_graphics::{
    draw_4x4_text, draw_centered_text, draw_point, draw_rect, set_drawing_colors, SCREEN_SIZE,
};
use wasm4fun_input::GamePad;
use wasm4fun_log::debug;
use wasm4fun_random::Generator;
use wasm4fun_time::Ticker;

use crate::audio::AudioQueue;

use super::{InGameState, State, Transition};

const MAX_COOLDOWN: u32 = 10;

use crate::assets::{BUTTON_SPRITE, LOGO_SPRITE};
use crate::game::HighScores;
use crate::graphics::draw_5x8_text;

pub struct MainMenuState {
    highscores: HighScores,
    cooldown: u32,
    step: u32,
}

impl MainMenuState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let highscores = HighScores::load();
        Self {
            highscores,
            cooldown: MAX_COOLDOWN,
            step: 0,
        }
    }

    pub fn draw(&self, _gamepad: Option<&GamePad>) {
        self.draw_logo();
        self.draw_highscores();
        self.draw_press_button_to_start();
        self.draw_credits();
    }

    pub fn update(&mut self, gamepad: &GamePad, _audio_queue: &mut AudioQueue) -> Transition {
        self.update_cooldowns();
        self.update_step();
        self.handle_input(gamepad)
    }

    fn update_cooldowns(&mut self) {
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
    }

    fn update_step(&mut self) {
        if Ticker.within_second() % 30 == 0 {
            self.step += 1;
            self.step %= 4;
        }
    }

    fn handle_input(&mut self, gamepad: &GamePad) -> Transition {
        if self.cooldown == 0 && gamepad.z() {
            // Necessary for when we return to this state
            self.cooldown = MAX_COOLDOWN;

            debug!("Start a new game");
            let generator = Generator::new_from_user_interaction();
            Transition::Push(State::InGame(InGameState::new(generator)))
        } else {
            Transition::Noop
        }
    }

    fn draw_logo(&self) {
        let x = 0;
        let y = 5;

        set_drawing_colors(0x03);
        LOGO_SPRITE.blit(x + 1, y + 1);
        LOGO_SPRITE.blit(x + 2, y + 2);

        set_drawing_colors(0x02);
        LOGO_SPRITE.blit(x, y);
    }

    fn draw_highscores(&self) {
        if self.highscores.is_empty() {
            return;
        }

        let text = "HIGH SCORES";
        let y = 36;

        set_drawing_colors(3);
        draw_centered_text(text, y);

        let mut buffer = [0; 128];

        let x = ((SCREEN_SIZE - 14 * 4) / 2) as i32;
        let y = y + 10;

        for (i, highscore) in self.highscores.iter().enumerate() {
            set_drawing_colors(0x02);
            draw_4x4_text(highscore.name(), x, y + i as i32 * 5);

            let text = format_i32_padded(&mut buffer, highscore.score() as i32, 5, ' ');
            set_drawing_colors(0x04);
            draw_4x4_text(text, x + 4 * 4, y + i as i32 * 5);
        }
    }

    fn draw_press_button_to_start(&self) {
        let text = "Press    to start";
        let height = 8;
        let width = text.len() as u32 * 5;
        let x = (SCREEN_SIZE - width) as i32 / 2;
        let y = 100;

        if Ticker.within_second() < 30 {
            set_drawing_colors(0x02);
        } else {
            set_drawing_colors(0x03);
        }
        draw_5x8_text(text, x, y);

        self.draw_margin(x - 2, y - 6, width + 2 + 1, height + 2 + 3 + 4);

        let src_x;
        let offset;
        if Ticker.within_second() < 30 {
            src_x = 0;
            offset = 0;
        } else {
            src_x = 16;
            offset = 1;
        };

        let button_x = x + 5 * 5 + 3;
        let button_y = y - 5;
        set_drawing_colors(0x1230);
        BUTTON_SPRITE
            .clip(src_x, 0, BUTTON_SPRITE.width() / 2, BUTTON_SPRITE.height())
            .blit(button_x, button_y);
        set_drawing_colors(0x03);
        draw_5x8_text("z", button_x + 5, button_y + 5 + offset);
    }

    fn draw_margin(&self, x: i32, y: i32, width: u32, height: u32) {
        set_drawing_colors(0x02);

        for i in 0..(width / 4) {
            let xi = x + 4 * i as i32 + self.step as i32;
            draw_point(xi, y);
            let xi = x + 4 * i as i32 + 4 - self.step as i32;
            draw_point(xi - 1, y + height as i32);
        }

        for i in 0..(height / 4) {
            let yi = y + 4 * i as i32 + 4 - self.step as i32;
            draw_point(x, yi);
            let yi = y + 4 * i as i32 + self.step as i32;
            draw_point(x + width as i32, yi);
        }
    }

    fn draw_credits(&self) {
        let line_height = 10;

        let x = 6;
        let y = SCREEN_SIZE as i32 - 3 * line_height - 2;
        let width = SCREEN_SIZE - x as u32 * 2;
        let height = line_height as u32 * 3;

        set_drawing_colors(0x33);
        draw_rect(x, y, width + 4, height + 1);

        set_drawing_colors(0x22);
        draw_rect(x - 2, y - 2, width + 4, height + 1);

        set_drawing_colors(0x03);

        let text = concat!("Version ", env!("CARGO_PKG_VERSION"));
        draw_5x8_text(&text, x, y);

        let text = "Developed by ";
        draw_5x8_text(&text, x, y + line_height);
        let offset = text.len() as i32 * 5;

        let text = env!("CARGO_PKG_AUTHORS")
            .split_once('<')
            .unwrap()
            .0
            .trim_end();
        draw_5x8_text(&text, x + offset, y + line_height);

        let text = "for WASM-4 JAM (January 2022)";
        draw_5x8_text(&text, x, y + line_height * 2);
    }
}
