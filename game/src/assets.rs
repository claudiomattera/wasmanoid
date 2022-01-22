// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm4fun_graphics::Sprite;

include!(concat!(env!("OUT_DIR"), "/sprites.rs"));

pub const BRICK_SPRITE: Sprite = Sprite::new(
    sprites::BRICK_WIDTH,
    sprites::BRICK_HEIGHT,
    sprites::BRICK_FLAGS,
    &sprites::BRICK,
);

pub const WALL_SPRITE: Sprite = Sprite::new(
    sprites::WALL_WIDTH,
    sprites::WALL_HEIGHT,
    sprites::WALL_FLAGS,
    &sprites::WALL,
);

pub const FONT4X8_SPRITE: Sprite = Sprite::new(
    sprites::FONT4X8_WIDTH,
    sprites::FONT4X8_HEIGHT,
    sprites::FONT4X8_FLAGS,
    &sprites::FONT4X8,
);

pub const BUTTON_SPRITE: Sprite = Sprite::new(
    sprites::BUTTON_WIDTH,
    sprites::BUTTON_HEIGHT,
    sprites::BUTTON_FLAGS,
    &sprites::BUTTON,
);

pub const BALL_SPRITE: Sprite = Sprite::new(
    sprites::BALL_WIDTH,
    sprites::BALL_HEIGHT,
    sprites::BALL_FLAGS,
    &sprites::BALL,
);

pub const BAR_SPRITE: Sprite = Sprite::new(
    sprites::BAR_WIDTH,
    sprites::BAR_HEIGHT,
    sprites::BAR_FLAGS,
    &sprites::BAR,
);

pub const LOGO_SPRITE: Sprite = Sprite::new(
    sprites::LOGO_WIDTH,
    sprites::LOGO_HEIGHT,
    sprites::LOGO_FLAGS,
    &sprites::LOGO,
);

pub const BONUS_SPRITE: Sprite = Sprite::new(
    sprites::BONUS_WIDTH,
    sprites::BONUS_HEIGHT,
    sprites::BONUS_FLAGS,
    &sprites::BONUS,
);
