// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(clippy::identity_op, clippy::erasing_op)]

use tinyvec::array_vec;
use tinyvec::ArrayVec;

use wasm4fun_fmt::format_i32_padded;
use wasm4fun_graphics::{draw_rect, set_drawing_colors, Rotation, SCREEN_SIZE};
use wasm4fun_input::GamePad;
use wasm4fun_log::debug;
use wasm4fun_random::Generator;
use wasm4fun_time::{Ticker, Timer};

use crate::assets::{BONUS_SPRITE, BRICK_SPRITE, WALL_SPRITE};
use crate::audio::{bonus_tone, game_over_tone, game_won_tone, AudioQueue};
use crate::game::{Ball, Bar, Bonus, Brick, HighScores};
use crate::graphics::draw_5x8_text;
use crate::math::normalize_vector;
use crate::ELAPSED_TIME_IN_SECONDS;

use super::{SaveScoreState, State, Transition};

const LEFT_WALL: i32 = 8;
const TOP_WALL: i32 = 16;
const TOP_MARGIN: i32 = 8;
const BAR_Y: i32 = SCREEN_SIZE as i32 - 30;
const BAR_HEIGHT: u32 = 6;
const MAX_BAR_SPEED: f32 = 200.0;
const MAX_BRICKS: usize = 30;
const BRICK_WIDTH: u32 = 24;
const BRICK_HEIGHT: u32 = 8;
const BRICK_INITIAL_HEALTH: u8 = 3;
const MAX_BONUSES: usize = 3;
const BONUS_SPEED: f32 = 100.0;
const BONUS_WIDTH: u32 = 8;
const BONUS_HEIGHT: u32 = 8;
const MAX_BAR_SECTIONS: u32 = 4;

pub struct InGameState {
    score: u32,
    timer: Timer,

    bar: Bar,
    ball: Ball,

    generator: Generator,

    bricks: ArrayVec<[Brick; MAX_BRICKS]>,
    bonuses: ArrayVec<[Bonus; MAX_BONUSES]>,
}

impl InGameState {
    pub fn new(mut generator: Generator) -> Self {
        let initial_ball_unit_velocity =
            normalize_vector((generator.gen_range(-1..1) as f32, -1.0));
        Self {
            score: 0,
            timer: Timer::new(),
            bar: Bar::new(),
            ball: Ball::new(initial_ball_unit_velocity),
            generator,
            bricks: array_vec!(
                [Brick; MAX_BRICKS] =>
                // First row
                (0 * BRICK_WIDTH as u8, 0 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (1 * BRICK_WIDTH as u8, 0 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (2 * BRICK_WIDTH as u8, 0 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (3 * BRICK_WIDTH as u8, 0 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (4 * BRICK_WIDTH as u8, 0 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (5 * BRICK_WIDTH as u8, 0 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),

                // Second row
                (0 * BRICK_WIDTH as u8, 1 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (1 * BRICK_WIDTH as u8, 1 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (2 * BRICK_WIDTH as u8, 1 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (3 * BRICK_WIDTH as u8, 1 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (4 * BRICK_WIDTH as u8, 1 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (5 * BRICK_WIDTH as u8, 1 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),

                // Third row
                (0 * BRICK_WIDTH as u8, 2 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (1 * BRICK_WIDTH as u8, 2 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (2 * BRICK_WIDTH as u8, 2 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (3 * BRICK_WIDTH as u8, 2 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (4 * BRICK_WIDTH as u8, 2 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (5 * BRICK_WIDTH as u8, 2 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),

                // Fourth row
                (0 * BRICK_WIDTH as u8, 3 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (1 * BRICK_WIDTH as u8, 3 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (2 * BRICK_WIDTH as u8, 3 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (3 * BRICK_WIDTH as u8, 3 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (4 * BRICK_WIDTH as u8, 3 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
                (5 * BRICK_WIDTH as u8, 3 * BRICK_HEIGHT as u8, BRICK_INITIAL_HEALTH),
            ),
            bonuses: array_vec!(),
        }
    }

    pub fn draw(&self, _gamepad: Option<&GamePad>) {
        self.clear_background();
        self.draw_dashboard();
        self.draw_walls();
        self.draw_bar();
        self.draw_ball();
        self.draw_bonuses();
        self.draw_bricks();
    }

    fn clear_background(&self) {
        set_drawing_colors(0x44);
        draw_rect(0, 0, SCREEN_SIZE, SCREEN_SIZE);
    }

    fn draw_dashboard(&self) {
        set_drawing_colors(0x01);

        let mut buffer = [0; 10];

        let label = "SCORE:";
        let score_digits = 6;
        draw_5x8_text(label, 0, 0);
        let text = format_i32_padded(&mut buffer, self.score as i32, score_digits, ' ');
        draw_5x8_text(text, label.len() as i32 * 5, 0);

        let label = "TIME:";
        let time_digits = 4;
        let x = SCREEN_SIZE as i32 - time_digits as i32 * 5 - label.len() as i32 * 5;
        draw_5x8_text(label, x, 0);
        let text = format_i32_padded(&mut buffer, self.timer.get() as i32, time_digits, ' ');
        draw_5x8_text(text, label.len() as i32 * 5 + x, 0);
    }

    fn draw_walls(&self) {
        set_drawing_colors(0x1234);
        let wall_size = 8;

        let top_wall = WALL_SPRITE
            .clip(0, 0, wall_size, wall_size)
            .rotate(Rotation::Rotate90);
        let left_wall = WALL_SPRITE.clip(0, 0, wall_size, wall_size);
        let right_wall = WALL_SPRITE
            .clip(0, 0, wall_size, wall_size)
            .rotate(Rotation::Rotate180)
            .flip_vertically(true);
        let top_left_corner = WALL_SPRITE.clip(wall_size, 0, wall_size, wall_size);
        let top_right_corner = WALL_SPRITE
            .clip(wall_size, 0, wall_size, wall_size)
            .flip_horizontally(true);
        top_left_corner.blit(0, TOP_MARGIN);
        top_right_corner.blit(SCREEN_SIZE as i32 - wall_size as i32, TOP_MARGIN);
        for i in 1..19 {
            top_wall.blit(i * wall_size as i32, TOP_MARGIN);
            left_wall.blit(0, TOP_MARGIN + i * wall_size as i32);
            right_wall.blit(
                SCREEN_SIZE as i32 - wall_size as i32,
                TOP_MARGIN + i * wall_size as i32,
            );
        }
        left_wall.blit(0, TOP_MARGIN + 19 * wall_size as i32);
        right_wall.blit(
            SCREEN_SIZE as i32 - wall_size as i32,
            TOP_MARGIN + 19 * wall_size as i32,
        );
    }

    fn draw_bar(&self) {
        self.bar.draw();
    }

    fn draw_ball(&self) {
        self.ball.draw();
    }

    fn draw_bonuses(&self) {
        let src_x = (4.0 * (Ticker.within_second() as f32 / 60.0)) as u32 * 8;

        for (x, y) in self.bonuses.iter() {
            BONUS_SPRITE.clip(src_x, 0, 8, 8).blit(*x as i32, *y as i32);
        }
    }

    fn draw_bricks(&self) {
        for (x, y, health) in self.bricks.iter() {
            set_drawing_colors(0x1230);
            BRICK_SPRITE
                .clip(0, 0, BRICK_SPRITE.width(), 8)
                .blit(*x as i32 + LEFT_WALL, *y as i32 + TOP_WALL);
            if *health < 3 {
                BRICK_SPRITE
                    .clip(0, 8, BRICK_SPRITE.width(), 8)
                    .blit(*x as i32 + LEFT_WALL, *y as i32 + TOP_WALL);
            }
            if *health < 2 {
                BRICK_SPRITE
                    .clip(0, 16, BRICK_SPRITE.width(), 8)
                    .blit(*x as i32 + LEFT_WALL, *y as i32 + TOP_WALL);
            }
        }
    }

    pub fn update(&mut self, gamepad: &GamePad, audio_queue: &mut AudioQueue) -> Transition {
        self.update_bar_position(gamepad);
        self.update_ball_position();
        self.update_bonuses_position();
        self.handle_collisions(audio_queue);
        self.catch_bonuses(audio_queue);
        self.remove_destroyed_bricks();
        self.remove_lost_bonuses();
        self.update_timer();
        self.ensure_ball_moves_vertically();
        self.normalize_ball_velocity();

        self.handle_game_over(audio_queue)
    }

    fn update_ball_position(&mut self) {
        self.ball.update_position();
    }

    fn update_bar_position(&mut self, gamepad: &GamePad) {
        self.bar.update_position(gamepad);
    }

    fn update_bonuses_position(&mut self) {
        for (_x, y) in self.bonuses.iter_mut() {
            *y += (BONUS_SPEED * ELAPSED_TIME_IN_SECONDS) as u8;
        }
    }

    pub fn handle_collisions(&mut self, audio_queue: &mut AudioQueue) {
        let (score, bonus) = self
            .ball
            .handle_collisions(&self.bar, &mut self.bricks, audio_queue);

        self.score += score;

        if let Some(bonus) = bonus {
            if self.bonuses.len() < self.bonuses.capacity() && self.generator.gen_range(0..10) < 5 {
                debug!("Generating a bonus");

                self.bonuses.push(bonus);
            }
        }
    }

    fn catch_bonuses(&mut self, audio_queue: &mut AudioQueue) {
        let bar_x1 = self.bar.position() as i32 - LEFT_WALL;
        let bar_y1 = BAR_Y;
        let bar_x2 = bar_x1 + 8 * (self.bar.sections() as i32 + 2);
        let bar_y2 = bar_y1 + BAR_HEIGHT as i32;

        let mut caught_bonuses = 0;

        for (x, y) in self.bonuses.iter_mut() {
            let bonus_x1 = *x as i32 - LEFT_WALL;
            let bonus_y1 = *y as i32;
            let bonus_x2 = bonus_x1 + BONUS_WIDTH as i32;
            let bonus_y2 = bonus_y1 + BONUS_HEIGHT as i32;

            let contained_x = (bar_x1 - 1..bar_x2 + 1).contains(&bonus_x1)
                || (bar_x1 - 1..bar_x2 + 1).contains(&bonus_x2);
            let contained_y = (bar_y1 - 1..bar_y2 + 1).contains(&bonus_y1)
                || (bar_y1 - 1..bar_y2 + 1).contains(&bonus_y2);
            if contained_x && contained_y {
                debug!("Bonus caught!!!");
                audio_queue.play(bonus_tone());
                caught_bonuses += 1;
                *y = SCREEN_SIZE as u8 + 10;
            }
        }

        for _ in 0..caught_bonuses {
            self.give_random_bonus();
        }
    }

    fn give_random_bonus(&mut self) {
        let bonus_type = if self.bar.sections() < MAX_BAR_SECTIONS
            && self.ball.strength() < BRICK_INITIAL_HEALTH
        {
            self.generator.gen_range(1..4)
        } else if self.bar.sections() < MAX_BAR_SECTIONS {
            1
        } else if self.ball.strength() < BRICK_INITIAL_HEALTH {
            2
        } else if self.bar.speed() < MAX_BAR_SPEED {
            3
        } else {
            0
        };

        match bonus_type {
            1 => {
                self.bar.increase_sections();
            }
            2 => {
                self.ball.increase_strength();
            }
            3 => {
                self.bar.increase_speed();
            }
            _ => {}
        }
    }

    fn remove_destroyed_bricks(&mut self) {
        self.bricks.retain(|(_x, _y, health)| *health > 0);
    }

    fn remove_lost_bonuses(&mut self) {
        self.bonuses.retain(|(_x, y)| *y < SCREEN_SIZE as u8);
    }

    fn update_timer(&mut self) {
        self.timer.update()
    }

    fn ensure_ball_moves_vertically(&mut self) {
        self.ball.ensure_moves_vertically();
    }

    fn normalize_ball_velocity(&mut self) {
        self.ball.normalize_velocity();
    }

    fn handle_game_over(&self, audio_queue: &mut AudioQueue) -> Transition {
        let game_over = self.ball_lost();
        let game_won = self.all_bricks_destroyed();

        if game_over {
            audio_queue.play(game_over_tone());
        }

        if game_won {
            audio_queue.play(game_won_tone());
            audio_queue.enqueue(10, game_won_tone().with_release(60));
        }

        if game_over || game_won {
            // If in single-player mode, possibly save a high score
            let highscores = HighScores::load();
            if highscores.is_beated_by(self.score) {
                Transition::Replace(State::SaveScore(SaveScoreState::new(self.score)))
            } else {
                Transition::Pop
            }
        } else {
            Transition::Noop
        }
    }

    fn ball_lost(&self) -> bool {
        self.ball.is_lost()
    }

    fn all_bricks_destroyed(&self) -> bool {
        self.bricks.is_empty()
    }
}
