use crate::constants::*;
use macroquad::prelude::*;
use macroquad::rand;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RGB8 {
    r: u8,
    g: u8,
    b: u8,
}

fn array_to_coord(input: usize, grid_size_x: usize, grid_size_y: usize) -> (usize, usize) {
    let x = input % grid_size_x;
    let y = input / grid_size_y;
    (x, y)
}

#[allow(dead_code)]
impl RGB8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn rgb_to_mq_color(&self) -> Color {
        Color {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
            a: 1.0,
        }
    }
    pub fn default() -> Self {
        Self {
            r: 50,
            g: 50,
            b: 50,
        }
    }
    pub fn new_rnd() -> Self {
        Self {
            r: rand::gen_range(0, 255),
            g: rand::gen_range(0, 255),
            b: rand::gen_range(0, 255),
        }
    }
}

pub struct Leds {
    grid: [[RGB8; GRID_HEIGHT]; GRID_WIDTH],
}

impl Leds {
    pub fn new() -> Self {
        Self {
            grid: [[RGB8::default(); 8]; 8],
        }
    }

    pub fn get_mut_ref_rgb8(&mut self, x: usize, y: usize) -> &mut RGB8 {
        &mut self.grid[x][y]
    }

    pub fn update_clockwise(&mut self, percentage: f32, color: RGB8) {
        let phase = (CLOCK.len() as f32 * percentage).ceil() as usize;
        for (x, y) in &CLOCK[0..phase] {
            *self.get_mut_ref_rgb8(*x, *y) = color;
        }
    }

    pub fn draw_image(&mut self, rgb_vector: Vec<(u8, u8, u8)>) {
        for (index, pixel) in rgb_vector.iter().enumerate() {
            let rgb = RGB8::new(pixel.0, pixel.1, pixel.2);
            let (x, y) = array_to_coord(index, GRID_WIDTH, GRID_HEIGHT);
            self.update_pixel(x, y, rgb);
        }
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, new_color: RGB8) {
        self.grid[x][y] = new_color;
    }

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

                draw_poly(x, y, 32, point_radius, 0.0, point.rgb_to_mq_color());
            }
        }
    }
}
