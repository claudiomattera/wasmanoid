// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tinyvec::array_vec;
use tinyvec::ArrayVec;

use wasm4fun_sound::{DutyCycle, Tone, WaveForm};

pub const MAX_TONES: usize = 10;

pub struct AudioQueue {
    tones: ArrayVec<[(u64, Tone); MAX_TONES]>,
}

impl AudioQueue {
    pub fn new() -> Self {
        Self {
            tones: array_vec!(),
        }
    }

    pub fn enqueue(&mut self, frame: u64, tone: Tone) {
        self.tones.push((frame, tone));
    }

    pub fn play(&mut self, tone: Tone) {
        self.enqueue(0, tone);
    }

    pub fn update(&mut self) {
        for (_, tone) in self.tones.iter().filter(|(frame, _)| *frame == 0) {
            tone.play();
        }

        self.tones.retain(|(frame, _)| *frame > 0);

        for (frame, _) in self.tones.iter_mut() {
            *frame -= 1;
        }
    }
}

pub fn bounce_tone() -> Tone {
    Tone::new()
        .with_first_frequency(300)
        .with_release(10)
        .with_volume(50)
        .with_wave_form(WaveForm::Triangle)
}

pub fn destroy_tone() -> Tone {
    Tone::new()
        .with_first_frequency(400)
        .with_second_frequency(700)
        .with_release(30)
        .with_volume(50)
        .with_wave_form(WaveForm::Noise)
}

pub fn bonus_tone() -> Tone {
    Tone::new()
        .with_first_frequency(0)
        .with_second_frequency(1000)
        .with_attack(12)
        .with_decay(12)
        .with_release(12)
        .with_volume(50)
        .with_wave_form(WaveForm::Triangle)
}

pub fn game_over_tone() -> Tone {
    Tone::new()
        .with_first_frequency(300)
        .with_second_frequency(10)
        .with_attack(20)
        .with_decay(70)
        .with_volume(50)
        .with_wave_form(WaveForm::Pulse1)
        .with_duty_cycle(DutyCycle::ThreeQuarters)
}

pub fn game_won_tone() -> Tone {
    Tone::new()
        .with_first_frequency(500)
        .with_release(20)
        .with_volume(50)
        .with_wave_form(WaveForm::Pulse1)
        .with_duty_cycle(DutyCycle::ThreeQuarters)
}

pub fn menu_move_tone() -> Tone {
    Tone::new()
        .with_first_frequency(1000)
        .with_second_frequency(300)
        .with_sustain(4)
        .with_volume(50)
        .with_wave_form(WaveForm::Triangle)
}

pub fn menu_select_tone() -> Tone {
    Tone::new()
        .with_first_frequency(300)
        .with_second_frequency(1000)
        .with_sustain(10)
        .with_volume(50)
        .with_wave_form(WaveForm::Triangle)
}
