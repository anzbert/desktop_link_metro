use crate::*;

struct Leds {
    grid: [[u8; constants::GRID_WIDTH]; constants::GRID_HEIGHT],
}

impl Leds {
    fn new() -> Self {
        Self { grid: [[0; 8]; 8] }
    }
    fn draw_centered(&self) {
        for row in self.grid {
            for point in row {}
        }
    }
}
