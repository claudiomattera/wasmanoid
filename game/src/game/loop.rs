// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::mem::MaybeUninit;

use wasm4fun_graphics::Palette;
use wasm4fun_input::GamePad;
use wasm4fun_time::Ticker;

use crate::statemachine::Machine;

use crate::audio::AudioQueue;

/// The game state machine
pub static mut STATE_MACHINE: MaybeUninit<Machine> = MaybeUninit::uninit();

/// The game audio queue
pub static mut AUDIO_QUEUE: MaybeUninit<AudioQueue> = MaybeUninit::uninit();

#[no_mangle]
fn start() {
    Palette::Default.set();

    // Initialize state machine
    let state_machine = unsafe { &mut STATE_MACHINE };
    state_machine.write(Machine::new());

    // Initialize audio queue
    let audio_queue = unsafe { &mut AUDIO_QUEUE };
    audio_queue.write(AudioQueue::new());
}

#[no_mangle]
fn update() {
    let state_machine = unsafe { &mut STATE_MACHINE };
    let state_machine = unsafe { state_machine.assume_init_mut() };

    let audio_queue = unsafe { &mut AUDIO_QUEUE };
    let audio_queue = unsafe { audio_queue.assume_init_mut() };

    let gamepad = GamePad::open(1);

    state_machine.draw(gamepad);
    state_machine.update(gamepad, audio_queue);

    audio_queue.update();

    Ticker.update();
}
