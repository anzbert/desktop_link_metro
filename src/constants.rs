pub const LATENCY_COMP: f64 = 0.075;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;

pub const GRID_HEIGHT: usize = 8;
pub const GRID_WIDTH: usize = 8;
pub const SCREEN_MARGIN: f32 = 0.9;
pub const POINT_MARGIN: f32 = 0.9;

#[allow(unused_allocation)]
pub const CLOCK: [(usize, usize); 28] = [
    (4, 0),
    (5, 0),
    (6, 0),
    (7, 0),
    (7, 1),
    (7, 2),
    (7, 3),
    (7, 4),
    (7, 5),
    (7, 6),
    (7, 7),
    (6, 7),
    (5, 7),
    (4, 7),
    (3, 7),
    (2, 7),
    (1, 7),
    (0, 7),
    (0, 6),
    (0, 5),
    (0, 4),
    (0, 3),
    (0, 2),
    (0, 1),
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
];

// dynamically size and create clock at compile time without using vectors:
// const fn create_clock_array_size() -> usize {
//     2 * GRID_HEIGHT + 2 * (GRID_WIDTH - 2)
// }
// const fn create_clock_array() -> [(usize, usize); create_clock_array_size()] {
//     [(0, 0); create_clock_array_size()]
// }

// fn get_clock_array() -> [(usize, usize); create_clock_array_size()] {
//     for i in 0..create_clock_array_size() {
//          for
//     }
// }
