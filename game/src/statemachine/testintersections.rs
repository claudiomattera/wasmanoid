// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm4fun_graphics::*;
use wasm4fun_input::GamePad;

use crate::audio::AudioQueue;

use crate::math::*;

use super::Transition;

pub struct TestIntersectionsState {}

impl TestIntersectionsState {
    #[allow(clippy::new_without_default)]
    #[allow(unused)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, _gamepad: Option<&GamePad>) {
        let (xs1, ys1) = (35, 38);
        let (xs2, ys2) = (85, 38);

        let (xs3, ys3) = (122, 38);
        let (xs4, ys4) = (122, 91);

        let r = 8;

        for xc in 0..SCREEN_SIZE {
            for yc in 0..SCREEN_SIZE {
                if intersects_horizontal_segment((xc as i32, yc as i32), r, xs1, xs2, ys1).is_some()
                    || intersects_vertical_segment((xc as i32, yc as i32), r, ys3, ys4, xs3)
                        .is_some()
                {
                    set_drawing_colors(0x01);
                } else {
                    set_drawing_colors(0x03);
                }

                draw_point(xc as i32, yc as i32);
            }
        }

        set_drawing_colors(0x02);
        draw_line(xs1, ys1, xs2, ys2);
        draw_line(xs3, ys3, xs4, ys4);
    }

    pub fn update(&self, _gamepad: &GamePad, _audio_queue: &mut AudioQueue) -> Transition {
        Transition::Noop
    }
}
