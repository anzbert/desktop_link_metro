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
    let link = ableton_link::Link::new(120.0);
    let clock = link.clock();
    let quantum = 4.0;

    let mut leds = vis::Leds::new();

    loop {
        clear_background(GRAY);

        // PROCESS INPUT:
        if let "macos" = std::env::consts::OS {
            if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
                return; // return from main -> quit
            }
        }

        // GET CURRENT TEMPO AND PHASE
        link.with_app_session_state(|session_state| {
            let time = clock.micros();
            let tempo = session_state.tempo();
            let playing = session_state.is_playing();
            let beat = session_state.beat_at_time(time, quantum);

            let phase = session_state.phase_at_time(time, quantum);

            let percentage = phase / quantum;

            println!(
                "playing={}, quantum={}, clock={}, tempo={}, beat={}, phase = {}",
                playing, quantum, time, tempo, beat, phase
            );

            // UPDATE RENDER:
            leds.update_clockwise(percentage as f32);
            leds.draw_centered();
        });

        next_frame().await
    }
}
