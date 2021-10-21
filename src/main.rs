// mod vis;

use macroquad::prelude::*;

// Macroquad WINDOW CONFIG:
fn window_conf() -> Conf {
    Conf {
        window_title: "boink".to_owned(),
        window_width: 800,
        window_height: 600,
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
    // let link = ableton_link::Link::new(120.0);

    loop {
        // PROCESS INPUT:
        if let "macos" = std::env::consts::OS {
            if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
                return; // return from main -> quit
            }
        }

        // GET CURRENT TEMPO AND PHASE
        // ...

        // UPDATE RENDER:
        clear_background(GRAY);

        next_frame().await
    }
}
