// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm4fun_input::GamePad;
use wasm4fun_logo::draw_logo;
use wasm4fun_time::Ticker;

use crate::audio::AudioQueue;

use super::Transition;

pub struct SplashScreenState {}

impl SplashScreenState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, _gamepad: Option<&GamePad>) {
        draw_logo(Ticker.since_startup());
    }

    pub fn update(&mut self, _gamepad: &GamePad, _audio_queue: &mut AudioQueue) -> Transition {
        let delay = if cfg!(feature = "skip") { 0 } else { 240 };

        if Ticker.since_startup() > delay {
            Transition::Pop
        } else {
            Transition::Noop
        }
    }
}
