// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm4fun_graphics::SCREEN_SIZE;

use crate::assets::FONT4X8_SPRITE;

pub fn draw_5x8_text(s: impl AsRef<str>, x: i32, y: i32) {
    let s = s.as_ref();

    const FONT_WIDTH: u32 = 5;
    const FONT_HEIGHT: u32 = 8;

    for (i, c) in s.chars().enumerate() {
        if c.is_ascii() {
            let u = c as u32;
            let index = u - 32;
            let row = index / 32;
            let column = index % 32;

            let x = x + (FONT_WIDTH as i32) * (i as i32);
            let src_x = (FONT_WIDTH as u32) * column;
            let src_y = (FONT_HEIGHT as u32) * row;

            FONT4X8_SPRITE
                .clip(src_x, src_y, FONT_WIDTH, FONT_HEIGHT)
                .blit(x, y);
        }
    }
}

/// Draw centered text with 5×8 font using the current colours
pub fn draw_centered_5x8_text<T>(s: T, y: i32)
where
    T: AsRef<str>,
{
    const FONT_WIDTH: u32 = 5;

    let s = s.as_ref();
    let x = ((SCREEN_SIZE - s.len() as u32 * FONT_WIDTH) / 2) as i32;
    draw_5x8_text(s, x, y)
}
