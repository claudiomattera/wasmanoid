// Copyright Claudio Mattera 2022.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(target_family = "wasm")]
use micromath::F32Ext;

pub fn intersects_vertical_line((xc, _yc): (i32, i32), r: u32, x_line: i32) -> bool {
    let radius = r as i32;
    let distance = (xc - x_line).abs();
    distance == radius
}

pub fn intersects_horizontal_line((_xc, yc): (i32, i32), r: u32, y_line: i32) -> bool {
    let radius = r as i32;
    let distance = (yc - y_line).abs();
    distance == radius
}

pub fn intersects_horizontal_segment(
    (xc, yc): (i32, i32),
    r: u32,
    xs1: i32,
    xs2: i32,
    ys: i32,
) -> Option<f32> {
    if xs1 < xc && xc <= xs2 {
        if intersects_horizontal_line((xc, yc), r, ys) {
            Some((xc - xs1) as f32 / (xs2 - xs1) as f32)
        } else {
            None
        }
    } else {
        if ((xs1 - r as i32)..(xs1 + 1)).contains(&xc) {
            if ((xc - xs1).pow(2) as f32 + (yc - ys).pow(2) as f32 - r.pow(2) as f32).abs() <= 6.0 {
                Some(0.9)
            } else {
                None
            }
        } else if ((xs2)..(xs2 + r as i32 + 1)).contains(&xc) {
            if ((xc - xs2).pow(2) as f32 + (yc - ys).pow(2) as f32 - r.pow(2) as f32).abs() <= 6.0 {
                Some(0.1)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn intersects_vertical_segment(
    (xc, yc): (i32, i32),
    r: u32,
    ys1: i32,
    ys2: i32,
    xs: i32,
) -> Option<f32> {
    if ys1 < yc && yc <= ys2 {
        if intersects_vertical_line((xc, yc), r, xs) {
            Some((yc - ys1) as f32 / (ys2 - ys1) as f32)
        } else {
            None
        }
    } else {
        if ((ys1 - r as i32)..(ys1 + 1)).contains(&yc) {
            if ((yc - ys1).pow(2) as f32 + (xc - xs).pow(2) as f32 - r.pow(2) as f32).abs() <= 6.0 {
                Some(0.9)
            } else {
                None
            }
        } else if ((ys2)..(ys2 + r as i32 + 1)).contains(&yc) {
            if ((yc - ys2).pow(2) as f32 + (xc - xs).pow(2) as f32 - r.pow(2) as f32).abs() <= 6.0 {
                Some(0.1)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn normalize_vector((x, y): (f32, f32)) -> (f32, f32) {
    let normalization_factor = 1.0 / norm((x, y));
    (x * normalization_factor, y * normalization_factor)
}

pub fn norm((x, y): (f32, f32)) -> f32 {
    (x.powi(2) + y.powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_horizontal() {
        let (x, y) = (1.0, 0.0);
        let n = norm((x, y));
        assert_eq!(n, 1.0);
    }

    #[test]
    fn test_norm_vertical() {
        let (x, y) = (0.0, -1.0);
        let n = norm((x, y));
        assert_eq!(n, 1.0);
    }

    #[test]
    fn test_norm_diagonal() {
        let (x, y) = (1.0, -1.0);
        let n = norm((x, y));
        assert_eq!(n, 2.0_f32.sqrt());
    }

    #[test]
    fn test_normalize_vector() {
        let (x, y) = (1.0, 1.0);
        let (x, y) = normalize_vector((x, y));
        assert_eq!((x, y), (2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0));
    }
}
