mod constants;
mod vis;

use macroquad::prelude::*;
// use nalgebra::*;

// MQ WINDOW CONFIG:
fn window_conf() -> Conf {
    Conf {
        window_title: "rusty metro".to_owned(),
        window_width: constants::WIDTH,
        window_height: constants::HEIGHT,
        high_dpi: false,
        fullscreen: false,
        // sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}

// MAIN:
#[macroquad::main(window_conf)]
async fn main() {
    let mut link = ableton_link::Link::new(120.0);
    link.enable(true);

    let clock = link.clock();
    let quantum = 4.0;

    let mut leds = vis::Leds::new();
    let mut last_phase: f64 = 99.;
    let mut color = vis::RGB8::new_rnd();

    loop {
        clear_background(GRAY);

        if let "macos" = std::env::consts::OS {
            if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
                return; // -> quit on macOS
            }
        }

        link.with_app_session_state(|session_state| {
            let time = clock.micros();
            let tempo = session_state.tempo();
            let playing = session_state.is_playing();
            let beat = session_state.beat_at_time(time, quantum);
            let peers = link.num_peers();
            let phase = session_state.phase_at_time(time, quantum);

            let percentage = phase / quantum;

            // println!(
            //     "play:{}, q:{:.2}, tempo:{:.2}, beat:{:.2}, phase:{:.2}, peers:{}",
            //     playing, quantum, tempo, beat, phase, peers
            // );

            if phase < last_phase {
                color = vis::RGB8::new_rnd();
            }
            last_phase = phase;

            // println!("{:?}", color);
            leds.update_clockwise(percentage as f32, color);
            leds.draw_centered();
        });

        next_frame().await
    }
}
