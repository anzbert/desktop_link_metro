use crate::constants::*;
use macroquad::prelude::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RGBA8 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RGBA8 {
    pub fn rgba_to_mq_color(&self) -> Color {
        Color {
            r: self.r as f32,
            g: self.g as f32,
            b: self.b as f32,
            a: self.a as f32,
        }
    }
    pub fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

pub struct Leds {
    grid: [[RGBA8; GRID_HEIGHT]; GRID_WIDTH],
}

impl Leds {
    pub fn new() -> Self {
        Self {
            grid: [[RGBA8::default(); 8]; 8],
        }
    }

    pub fn update_clockwise(&mut self, percentage: f32) {}

    pub fn draw_centered(&self) {
        let smaller_side = if WIDTH <= HEIGHT { WIDTH } else { HEIGHT };
        let drawing_square = smaller_side as f32 * SCREEN_MARGIN;

        let distance_between_points = drawing_square / GRID_WIDTH as f32;
        let point_radius = distance_between_points as f32 * POINT_MARGIN / 2.0;

        let origin_x = WIDTH as f32 / 2.0 - drawing_square / 2.0 + distance_between_points / 2.0;
        let origin_y = HEIGHT as f32 / 2.0 - drawing_square / 2.0 + distance_between_points / 2.0;

        for (x, column) in self.grid.iter().enumerate() {
            for (y, point) in column.iter().enumerate() {
                let x = origin_x + distance_between_points * x as f32;
                let y = origin_y + distance_between_points * y as f32;

                draw_poly(x, y, 12, point_radius, 0.0, point.rgba_to_mq_color());
            }
        }
    }
}
