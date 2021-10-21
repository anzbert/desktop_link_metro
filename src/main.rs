mod constants;
mod vis;

use macroquad::prelude::*;

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

    loop {
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
            println!(
                "playing={}, quantum={}, clock={}, tempo={}, beat={}",
                playing, quantum, time, tempo, beat
            );

            // UPDATE RENDER:
            clear_background(GRAY);

            // draw LEDS
        });

        next_frame().await
    }
}
