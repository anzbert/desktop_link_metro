mod constants;
mod vis;

use ableton_link::{Clock, SessionState};
use gif2json::RgbaImageData;
use macroquad::prelude::*;

use crate::constants::LATENCY_COMP;
use egui_macroquad::*;

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

    let mut link_enabled = true;
    link.enable_start_stop_sync(true);

    let clock = link.clock();
    let mut quantum = 4.0;

    let mut tempo: f64 = 0.;
    let mut last_tempo: f64 = 0.;

    let mut play = false;
    let mut last_play = play;

    // Init Session Values:
    link.with_app_session_state(|ss| {
        tempo = ss.tempo();
        last_tempo = tempo;
    });

    let mut leds = vis::Leds::new();
    let mut color = vis::RGB8::new_rnd();

    // let mut last_phase: f64 = 999.;
    let mut last_beat: f64 = 0.0;

    let gif_counter =
        RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap();
    let gif_clock = RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap();
    let gif_rows = RgbaImageData::new_from_bytes(include_bytes!("../img/rows_alpha.gif")).unwrap();
    let gif_circular =
        RgbaImageData::new_from_bytes(include_bytes!("../img/circular.gif")).unwrap();

    loop {
        clear_background(Color::new(0.20, 0.20, 0.20, 1.0));

        if let "macos" = std::env::consts::OS {
            if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
                return; // -> quit on macOS
            }
        }

        link.with_app_session_state(|session_state| {
            let time = clock.micros();
            tempo = session_state.tempo();
            let playing = session_state.is_playing();
            let beat = session_state.beat_at_time(time, quantum);
            let _peers = link.num_peers();
            let phase = session_state.phase_at_time(time, quantum);

            // let frame_delay = get_frame_time();
            // 60,000 / BPM = BPMS

            let compensated_phase = phase + LATENCY_COMP;
            let compensated_beat = beat + LATENCY_COMP;

            println!(
                "playing:{}, q:{:.2}, tempo:{:.2}, beat:{:.2}, phase:{:.2}, peers:{}",
                playing, quantum, tempo, beat, phase, _peers
            );

            // leds.draw_image(
            //     gif_circular
            //         .get_frame_vec_ref((compensated_phase * 4.0) as usize)
            //         .unwrap_or_else(|| gif_circular.get_frame_vec_ref(0).unwrap())
            //         .clone(),
            // );
            if playing {
                leds.draw_image(
                    gif_rows
                        .get_frame_vec_ref((compensated_phase * 2.0) as usize)
                        .unwrap_or_else(|| gif_rows.get_frame_vec_ref(0).unwrap())
                        .clone(),
                );
                leds.draw_image(
                    gif_counter
                        .get_frame_vec_ref(compensated_phase as usize)
                        .unwrap_or_else(|| gif_counter.get_frame_vec_ref(0).unwrap())
                        .clone(),
                );
            }
            // if phase < last_phase {
            //     color = vis::RGB8::new_rnd();
            // }
            // last_phase = phase;

            if compensated_beat - last_beat >= 1.0 {
                last_beat = compensated_beat.floor(); // re-calibrate to full beat
                color = vis::RGB8::new_rnd();
            }

            // let percentage = compensated_phase / quantum;
            // leds.update_clockwise(percentage as f32, color);
            leds.draw_centered();
        });

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("rusty_chain_link")
                .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
                .auto_sized()
                .min_width(constants::WIDTH as f32 * 0.15)
                .show(egui_ctx, |ui| {
                    ui.add(
                        egui::Slider::new(&mut quantum, 1.0..=8.0)
                            .integer()
                            .text("quantum"),
                    );
                    ui.add(egui::Checkbox::new(&mut link_enabled, "link enabled?"));
                    ui.add(egui::Checkbox::new(&mut play, "start/stop?"));
                    ui.add(
                        egui::Slider::new(&mut tempo, 20.0..=999.0)
                            .integer()
                            .text("bpm"),
                    );
                });
        });

        // extern "C" fn callback(x: bool) {
        //     println!("callback {}", x)
        // }
        // link.set_start_stop_callback(callback);

        if link_enabled {
            link.enable(true);
        } else {
            link.enable(false);
        }

        if !last_tempo.eq(&tempo) {
            link.with_app_session_state(|mut ff| {
                ff.set_tempo(tempo, clock.micros());
                ff.commit();
            });
        }
        if !last_play.eq(&play) {
            link.with_app_session_state(|mut ff| {
                ff.set_is_playing(play, clock.micros());

                ff.commit();
            });
        }

        last_play = play;
        last_tempo = tempo;

        egui_macroquad::draw();
        next_frame().await
    }
}
