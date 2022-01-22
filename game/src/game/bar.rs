// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm4fun_graphics::{set_drawing_colors, SCREEN_SIZE};
use wasm4fun_input::GamePad;

use crate::assets::BAR_SPRITE;
use crate::ELAPSED_TIME_IN_SECONDS;

const LEFT_WALL: i32 = 8;
const TOP_WALL: i32 = 16;
const BOARD_WIDTH: i32 = 144;
const INITIAL_BAR_SECTIONS: u32 = 1;
const BAR_Y: i32 = SCREEN_SIZE as i32 - 30;
const INITIAL_BAR_SPEED: f32 = 100.0;

pub struct Bar {
    position: f32,
    sections: u32,
    speed: f32,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            position: (SCREEN_SIZE - (INITIAL_BAR_SECTIONS + 2) * 8) as f32 / 2.0,
            sections: INITIAL_BAR_SECTIONS,
            speed: INITIAL_BAR_SPEED,
        }
    }

    pub fn width(&self) -> u32 {
        (self.sections + 2) * 8
    }

    pub fn height(&self) -> u32 {
        6
    }

    pub fn update_position(&mut self, gamepad: &GamePad) {
        if gamepad.left() {
            self.position -= self.speed * ELAPSED_TIME_IN_SECONDS;
        } else if gamepad.right() {
            self.position += self.speed * ELAPSED_TIME_IN_SECONDS;
        }
        self.position = self.position.clamp(
            LEFT_WALL as f32,
            (BOARD_WIDTH - (self.sections + 2) as i32 * 8 + LEFT_WALL) as f32,
        );
    }

    pub fn position(&self) -> f32 {
        self.position
    }

    pub fn sections(&self) -> u32 {
        self.sections
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn increase_sections(&mut self) {
        self.sections += 1;
    }

    pub fn increase_speed(&mut self) {
        self.speed *= 1.1;
    }

    pub fn draw(&self) {
        let bar_x = self.position as i32;
        let bar_y = BAR_Y + TOP_WALL - 2;

        set_drawing_colors(0x1230);

        BAR_SPRITE.clip(0, 0, 8, 8).blit(bar_x, bar_y);
        for i in 0..self.sections {
            BAR_SPRITE
                .clip(8, 0, 8, 8)
                .blit(bar_x + (i as i32 + 1) * 8, bar_y);
        }
        BAR_SPRITE
            .clip(0, 0, 8, 8)
            .flip_horizontally(true)
            .blit(bar_x + (self.sections as i32 + 1) * 8, bar_y);
    }
}
