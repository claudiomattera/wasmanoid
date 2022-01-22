// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(target_family = "wasm")]
use micromath::F32Ext;

use wasm4fun_graphics::{set_drawing_colors, Rotation, SCREEN_SIZE};
use wasm4fun_log::debug;
use wasm4fun_time::Ticker;

use crate::assets::BALL_SPRITE;
use crate::audio::{bounce_tone, destroy_tone, AudioQueue};
use crate::math::{
    intersects_horizontal_line, intersects_horizontal_segment, intersects_vertical_line,
    intersects_vertical_segment, normalize_vector,
};
use crate::ELAPSED_TIME_IN_SECONDS;

use super::{Bar, Bonus, Brick};

const LEFT_WALL: i32 = 8;
const TOP_WALL: i32 = 16;
const BOARD_WIDTH: i32 = 144;
const BAR_Y: i32 = SCREEN_SIZE as i32 - 30;
const INITIAL_BALL_SPEED: f32 = 80.0;
const INITIAL_BALL_STRENGTH: u8 = 1;
const BALL_RADIUS: u32 = 4;
const BRICK_WIDTH: u32 = 24;
const BRICK_HEIGHT: u32 = 8;
const MINIMAL_VERTICAL_VELOCITY: f32 = 0.1;
const MAX_BOUNCE_CALLBACK: u8 = 10;

pub struct Ball {
    coordinates: (f32, f32),
    speed: f32,
    unit_velocity: (f32, f32),
    strength: u8,

    bounce_callback: u8,
}

impl Ball {
    pub fn new(initial_ball_unit_velocity: (f32, f32)) -> Self {
        Self {
            coordinates: (
                (SCREEN_SIZE - BALL_RADIUS) as f32 / 2.0,
                BAR_Y as f32 - 10.0,
            ),
            speed: INITIAL_BALL_SPEED,
            unit_velocity: initial_ball_unit_velocity,
            strength: INITIAL_BALL_STRENGTH,
            bounce_callback: 0,
        }
    }

    pub fn is_lost(&self) -> bool {
        self.coordinates.1 > SCREEN_SIZE as f32
    }

    pub fn strength(&self) -> u8 {
        self.strength
    }

    pub fn increase_strength(&mut self) {
        self.strength += 1;
    }

    pub fn normalize_velocity(&mut self) {
        let (x, y) = normalize_vector(self.unit_velocity);
        self.unit_velocity = (x, y);
    }

    pub fn ensure_moves_vertically(&mut self) {
        if self.unit_velocity.1.abs() < MINIMAL_VERTICAL_VELOCITY {
            debug!("Ball is stuck bouncing horizontally");
            debug!("Let's nudge it");
            self.unit_velocity.1 = MINIMAL_VERTICAL_VELOCITY;
        }
    }

    pub fn update_position(&mut self) {
        let (x, y) = &mut self.coordinates;
        let (dx, dy) = self.unit_velocity;
        let dx = (self.speed * dx * ELAPSED_TIME_IN_SECONDS).min(1.0);
        let dy = (self.speed * dy * ELAPSED_TIME_IN_SECONDS).min(1.0);
        *x += dx;
        *y += dy;
    }

    pub fn draw(&self) {
        let (x, y) = self.coordinates;
        let (x, y) = (x as i32 + LEFT_WALL, y as i32 + TOP_WALL);

        let src_x = (8.0 * (Ticker.within_second() as f32 / 60.0)) as u32 * 8;
        let src_y;
        let rotation;
        if self.unit_velocity.0.abs() > self.unit_velocity.1.abs() + 0.5 {
            // Ball is rolling horizontally
            src_y = 0;
            if self.unit_velocity.0 > 0.0 {
                // Ball is rolling left to right
                rotation = Rotation::Rotate90;
            } else {
                // Ball is rolling right to left
                rotation = Rotation::Rotate270;
            }
        } else if self.unit_velocity.1.abs() > self.unit_velocity.0.abs() + 0.5 {
            // Ball is rolling vertically
            src_y = 0;
            if self.unit_velocity.1 < 0.0 {
                // Ball is rolling down to up
                rotation = Rotation::Rotate0;
            } else {
                // Ball is rolling up to down
                rotation = Rotation::Rotate180;
            }
        } else {
            // Ball is rolling diagonally
            src_y = 8;
            if self.unit_velocity.0 < 0.0 && self.unit_velocity.1 < 0.0 {
                // Ball is rolling up-left-ward
                rotation = Rotation::Rotate270;
            } else if self.unit_velocity.0 < 0.0 && self.unit_velocity.1 > 0.0 {
                // Ball is rolling down-left-ward
                rotation = Rotation::Rotate180;
            } else if self.unit_velocity.0 > 0.0 && self.unit_velocity.1 > 0.0 {
                // Ball is rolling up-right-ward
                rotation = Rotation::Rotate90;
            } else {
                // Ball is rolling down-right-ward
                rotation = Rotation::Rotate0;
            }
        }

        set_drawing_colors(0x1230);
        BALL_SPRITE
            .clip(src_x as u32, src_y, 8, 8)
            .rotate(rotation)
            .blit(x - BALL_RADIUS as i32, y - BALL_RADIUS as i32);
    }

    pub fn handle_collisions(
        &mut self,
        bar: &Bar,
        bricks: &mut [Brick],
        audio_queue: &mut AudioQueue,
    ) -> (u32, Option<Bonus>) {
        let score_and_bonus = if self.bounce_callback == 0 {
            self.handle_collisions_with_bar(bar, audio_queue);
            self.handle_collisions_with_bricks(bricks, audio_queue)
        } else {
            (0, None)
        };

        self.handle_collisions_with_walls(audio_queue);

        if self.bounce_callback > 0 {
            self.bounce_callback -= 1;
        }

        score_and_bonus
    }

    fn handle_collisions_with_bricks(
        &mut self,
        bricks: &mut [Brick],
        audio_queue: &mut AudioQueue,
    ) -> (u32, Option<Bonus>) {
        let (bx, by) = (self.coordinates.0 as i32, self.coordinates.1 as i32);

        let mut score = 0;
        let mut bonus = None;

        for (_i, (x, y, health)) in bricks.iter_mut().enumerate() {
            let left = *x as i32;
            let right = *x as i32 + BRICK_WIDTH as i32;
            let top = *y as i32;
            let bottom = *y as i32 + BRICK_HEIGHT as i32;

            let health = intersects_horizontal_segment((bx, by), BALL_RADIUS, left, right, top)
                .map(|ratio| {
                    debug!("Bounce upward with ratio {}", ratio);
                    self.unit_velocity.1 *= -1.0;
                    *health -= self.strength.min(*health);
                    *health
                })
                .or_else(|| {
                    intersects_horizontal_segment((bx, by), BALL_RADIUS, left, right, bottom).map(
                        |ratio| {
                            debug!("Bounce downward with ratio {}", ratio);
                            self.unit_velocity.1 *= -1.0;
                            *health -= self.strength.min(*health);
                            *health
                        },
                    )
                })
                .or_else(|| {
                    intersects_vertical_segment((bx, by), BALL_RADIUS, top, bottom, left).map(
                        |ratio| {
                            debug!("Bounce leftward with ratio {}", ratio);
                            self.unit_velocity.0 *= -1.0;
                            *health -= self.strength.min(*health);
                            *health
                        },
                    )
                })
                .or_else(|| {
                    intersects_vertical_segment((bx, by), BALL_RADIUS, top, bottom, right).map(
                        |ratio| {
                            debug!("Bounce rightward with ratio {}", ratio);
                            self.unit_velocity.0 *= -1.0;
                            *health -= self.strength.min(*health);
                            *health
                        },
                    )
                });

            match health {
                Some(0) => {
                    self.bounce_callback = MAX_BOUNCE_CALLBACK;
                    debug!("Brick destroyed");
                    audio_queue.play(destroy_tone());

                    score += 100;
                    bonus = Some((*x + BRICK_WIDTH as u8 / 2, *y + BRICK_HEIGHT as u8 / 2));
                }
                Some(_) => {
                    self.bounce_callback = MAX_BOUNCE_CALLBACK;
                    audio_queue.play(bounce_tone());

                    score += 10;
                }
                None => {}
            }
        }

        (score, bonus)
    }

    fn handle_collisions_with_walls(&mut self, audio_queue: &mut AudioQueue) {
        let (bx, by) = (self.coordinates.0 as i32, self.coordinates.1 as i32);

        if self.unit_velocity.1 < 0.0 && intersects_horizontal_line((bx, by), BALL_RADIUS, 0) {
            // Bounce downward
            debug!("Bounce downward from wall");
            debug!("Ball coordinates were {}x{}", bx, by);
            audio_queue.play(bounce_tone());
            self.unit_velocity.1 *= -1.0;
        } else if self.unit_velocity.0 < 0.0 && intersects_vertical_line((bx, by), BALL_RADIUS, 0) {
            // Bounce leftward
            debug!("Bounce leftward from wall");
            debug!("Ball coordinates were {}x{}", bx, by);
            audio_queue.play(bounce_tone());
            self.unit_velocity.0 *= -1.0;
        } else if self.unit_velocity.0 > 0.0
            && intersects_vertical_line((bx, by), BALL_RADIUS, BOARD_WIDTH as i32)
        {
            // Bounce rightward
            debug!("Bounce rightward from wall");
            debug!("Ball coordinates were {}x{}", bx, by);
            audio_queue.play(bounce_tone());
            self.unit_velocity.0 *= -1.0;
        }
    }

    fn handle_collisions_with_bar(&mut self, bar: &Bar, audio_queue: &mut AudioQueue) {
        let (bx, by) = (self.coordinates.0 as i32, self.coordinates.1 as i32);

        let (x, y) = (bar.position() as i32, BAR_Y);
        let left = x as i32;
        let right = x as i32 + bar.width() as i32;
        let center_x = x as i32 + bar.width() as i32 / 2;
        let top = y as i32;
        // let bottom = y as i32 + bar.height() as i32;
        let center_y = y as i32 + bar.height() as i32 / 2;

        let bounced =
            intersects_horizontal_segment((bx, by), BALL_RADIUS, left, right, top).map(|ratio| {
                debug!("Bounce upward from bar with ratio {}", ratio);
                let hit_x = left as f32 + (bar.width() as f32 * (1.0 - ratio));
                let hit_y = top as f32;
                self.unit_velocity.0 = hit_x - center_x as f32;
                self.unit_velocity.1 = hit_y - center_y as f32;
            });
        // .or_else(|| {
        //     intersects_horizontal_segment((bx, by), BALL_RADIUS, left, right, bottom).map(
        //         |ratio| {
        //             debug!("Bounce downward from bar with ratio {}", ratio);
        //             let hit_x = left as f32 + (bar.width() as f32 * ratio);
        //             let hit_y = bottom as f32;
        //             self.unit_velocity.0 = hit_x - center_x as f32;
        //             self.unit_velocity.1 = hit_y - center_y as f32;
        //         },
        //     )
        // })
        // .or_else(|| {
        //     intersects_vertical_segment((bx, by), BALL_RADIUS, top, bottom, left).map(|ratio| {
        //         debug!("Bounce leftward from bar with ratio {}", ratio);
        //         let hit_x = left as f32;
        //         let hit_y = top as f32 + (bar.height() as f32 * ratio);
        //         self.unit_velocity.0 = hit_x - center_x as f32;
        //         self.unit_velocity.1 = hit_y - center_y as f32;
        //     })
        // })
        // .or_else(|| {
        //     intersects_vertical_segment((bx, by), BALL_RADIUS, top, bottom, right).map(
        //         |ratio| {
        //             debug!("Bounce rightward from bar with ratio {}", ratio);
        //         let hit_x = right as f32;
        //         let hit_y = top as f32 + (bar.height() as f32 * (1.0 - ratio));
        //         self.unit_velocity.0 = hit_x - center_x as f32;
        //         self.unit_velocity.1 = hit_y - center_y as f32;
        //         },
        //     )
        // });

        if bounced.is_some() {
            self.bounce_callback = MAX_BOUNCE_CALLBACK;
            audio_queue.play(bounce_tone());
        }
    }
}
