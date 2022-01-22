// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! State machine data types and functions

use tinyvec::array_vec;
use tinyvec::ArrayVec;

use wasm4fun_input::GamePad;
use wasm4fun_log::debug;

use crate::audio::AudioQueue;

mod ingame;
use ingame::InGameState;

mod initial;
use initial::InitialState;

mod mainmenu;
use mainmenu::MainMenuState;

mod savescore;
use savescore::SaveScoreState;

mod splashscreen;
use splashscreen::SplashScreenState;

mod testintersections;
use testintersections::TestIntersectionsState;

const MAX_STATES: usize = 5;

/// Main stackable state machine
///
/// The game state is modelled as a stackable state machine.
/// The current state is the one on top of the stack.
/// Three kinds of transition can happen:
///
/// * The current state can switch to a different state;
/// * A new state can be pushed to the stack;
/// * The current state can be popped from the stack.
pub struct Machine {
    states_stack: ArrayVec<[State; MAX_STATES]>,
}

impl Machine {
    /// Create a new stack machine
    pub fn new() -> Self {
        let states_stack = array_vec!([State; MAX_STATES] => State::Initial(InitialState::new()));
        Machine { states_stack }
    }

    /// Draw all states in the stack
    pub fn draw(&self, gamepad: &GamePad) {
        for (i, state) in self.states_stack.iter().enumerate() {
            let gamepad = if i == self.states_stack.len() - 1 {
                Some(gamepad)
            } else {
                None
            };
            state.draw(gamepad);
        }
    }

    /// Update the top state on the stack
    ///
    /// The update returns a transition, which might change the content of the
    /// stack.
    ///
    /// The top state is always popped from the stack.
    /// If the current state wants to remain on the stack, it must return a
    /// [`Transition::Replace`] transition containing itself.
    pub fn update(&mut self, gamepad: &GamePad, audio_queue: &mut AudioQueue) {
        let stack_size = self.states_stack.len();

        let state: &mut State = self
            .states_stack
            .last_mut()
            .expect("Empty state machine!!!");
        let transition: Transition = state.update(gamepad, audio_queue);

        match transition {
            Transition::Replace(new_state) => {
                debug!("There are {} states in the stack", stack_size);
                debug!(
                    "Replacing state {} with state {}",
                    state.name(),
                    new_state.name()
                );
                self.states_stack.pop();
                self.states_stack.push(new_state);
                for state in &self.states_stack {
                    debug!("  - {}", state.name());
                }
            }
            Transition::Push(new_state) => {
                debug!("There are {} states in the stack", stack_size);
                debug!("Pushing new state {} to stack", new_state.name());
                self.states_stack.push(new_state);
                for state in &self.states_stack {
                    debug!("  - {}", state.name());
                }
            }
            Transition::PushN(new_states) => {
                debug!("There are {} states in the stack", stack_size);
                for new_state in new_states {
                    debug!("Pushing new state {} to stack", new_state.name());
                    self.states_stack.push(new_state);
                }
                for state in &self.states_stack {
                    debug!("  - {}", state.name());
                }
            }
            Transition::Pop => {
                debug!("There are {} states in the stack", stack_size);
                debug!("Popping state {} from stack", state.name());
                self.states_stack.pop();
                debug!(
                    "New top is {}",
                    self.states_stack.iter().last().unwrap().name()
                );
                for state in &self.states_stack {
                    debug!("  - {}", state.name());
                }
            }
            Transition::PopN(n) => {
                debug!("There are {} states in the stack", stack_size);
                debug!("Popping {} states from stack", n);
                for _ in 0..n {
                    let state: &mut State = self
                        .states_stack
                        .last_mut()
                        .expect("Empty state machine!!!");
                    debug!("Popping state {} from stack", state.name());
                    self.states_stack.pop();
                }
                debug!(
                    "New top is {}",
                    self.states_stack.iter().last().unwrap().name()
                );
                for state in &self.states_stack {
                    debug!("  - {}", state.name());
                }
            }
            Transition::Noop => {}
        }
    }
}

/// A state transition
pub enum Transition {
    /// The current state is replaced with a new state
    #[allow(unused)]
    Replace(State),

    /// A new state is pushed on top of the current state
    #[allow(unused)]
    Push(State),

    /// N new states are pushed on top of the current state
    #[allow(unused)]
    PushN(ArrayVec<[State; 2]>),

    /// The current state is popped from the stack
    #[allow(unused)]
    Pop,

    /// The current n states are popped from the stack
    #[allow(unused)]
    PopN(usize),

    /// The stack does not change
    #[allow(unused)]
    Noop,
}

/// A game state
///
/// Each state maintains its own state data, which is also responsible for
/// drawing and updating itself.
#[allow(unused)]
pub enum State {
    /// An invalid state
    Invalid,

    /// The initial state, created at the state machine initialization
    Initial(InitialState),

    // The splash screen state
    SplashScreen(SplashScreenState),

    /// The main menu state
    MainMenu(MainMenuState),

    /// The in-game state
    InGame(InGameState),

    /// The high-score state
    SaveScore(SaveScoreState),

    /// The test-intersections state
    TestIntersections(TestIntersectionsState),
}

impl State {
    /// Return the name of the state
    pub fn name(&self) -> &'static str {
        match self {
            State::Invalid => "invalid",
            State::Initial(_) => "initial",
            State::SplashScreen(_) => "splashscreen",
            State::MainMenu(_) => "main_menu",
            State::InGame(_) => "in_game",
            State::SaveScore(_) => "save_score",
            State::TestIntersections(_) => "test_intersections",
        }
    }

    /// Draw the current state
    ///
    /// This function delegates the drawing to the state data.
    pub fn draw(&self, gamepad: Option<&GamePad>) {
        match self {
            State::Invalid => panic!(),
            State::Initial(s) => s.draw(gamepad),
            State::SplashScreen(s) => s.draw(gamepad),
            State::MainMenu(s) => s.draw(gamepad),
            State::InGame(s) => s.draw(gamepad),
            State::SaveScore(s) => s.draw(gamepad),
            State::TestIntersections(s) => s.draw(gamepad),
        }
    }

    /// Update the current state
    ///
    /// This function delegates the update to the state data.
    pub fn update(&mut self, gamepad: &GamePad, audio_queue: &mut AudioQueue) -> Transition {
        match self {
            State::Invalid => panic!(),
            State::Initial(state) => state.update(gamepad, audio_queue),
            State::SplashScreen(state) => state.update(gamepad, audio_queue),
            State::MainMenu(state) => state.update(gamepad, audio_queue),
            State::InGame(state) => state.update(gamepad, audio_queue),
            State::SaveScore(state) => state.update(gamepad, audio_queue),
            State::TestIntersections(state) => state.update(gamepad, audio_queue),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Invalid
    }
}
