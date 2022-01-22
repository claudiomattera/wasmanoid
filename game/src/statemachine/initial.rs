// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyvec::array_vec;

use wasm4fun_input::GamePad;

use crate::audio::AudioQueue;

use super::{MainMenuState, SplashScreenState, State, TestIntersectionsState, Transition};

pub struct InitialState {}

impl InitialState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, _gamepad: Option<&GamePad>) {}

    pub fn update(&self, _gamepad: &GamePad, _audio_queue: &mut AudioQueue) -> Transition {
        if cfg!(feature = "test-intersections") {
            Transition::Replace(State::TestIntersections(TestIntersectionsState::new()))
        } else {
            Transition::PushN(array_vec!([State; 2] =>
                State::MainMenu(MainMenuState::new()),
                State::SplashScreen(SplashScreenState::new()),
            ))
        }
    }
}
