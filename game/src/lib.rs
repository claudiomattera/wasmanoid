// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![no_std]

#[cfg(target_family = "wasm")]
#[allow(unused)]
use wasm4fun_panichandler::*;

#[cfg(target_family = "wasm")]
#[cfg(feature = "buddy-alloc")]
mod alloc;

mod audio;

mod assets;

mod game;

mod graphics;

mod math;

mod statemachine;

/// The time elapsed since the previous frame
///
/// Since the WASM-4 console uses a constant framerate of 60 Hz, this time is
/// also constant.
pub const ELAPSED_TIME_IN_SECONDS: f32 = 0.016;
