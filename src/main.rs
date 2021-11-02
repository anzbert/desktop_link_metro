mod constants;
mod input;
mod vis;

use gif2json::RgbaImageData;
use macroquad::prelude::*;

use egui_macroquad::*;

// MQ WINDOW CONFIG:
fn window_conf() -> Conf {
    Conf {
        window_title: "rusty_metro".to_owned(),
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
    // Init LINK:
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

    let mut last_beat: f64 = 0.0;
    // let mut last_phase: f64 = 999.;

    let mut latency_comp = 0.05;

    link.with_app_session_state(|ss| {
        tempo = ss.tempo();
        last_tempo = tempo;
    });

    // Init VISUALS:
    #[derive(PartialEq, Debug)]
    enum Vis {
        Off,
        One,
        Two,
        Three,
    }
    let mut vis_selected = Vis::Off;

    let mut vis_numbers = true;

    let mut leds = vis::Leds::new();
    let mut color = vis::RGB8::new_rnd();

    let gif_counter =
        RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap();
    // let gif_clock = RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap();
    let gif_rows = RgbaImageData::new_from_bytes(include_bytes!("../img/rows_alpha.gif")).unwrap();
    let gif_circular =
        RgbaImageData::new_from_bytes(include_bytes!("../img/circular.gif")).unwrap();

    // ----------------------------------------------------------------------------------------------------------------
    // MAIN LOOP
    loop {
        clear_background(Color::new(0.20, 0.20, 0.20, 1.0));

        // GET KEYBOARD INPUT
        input::check_keyboard_input();

        // GET CURRENT SESSION STATE:
        link.with_app_session_state(|session_state| {
            let time = clock.micros();
            tempo = session_state.tempo();
            play = session_state.is_playing();
            let beat = session_state.beat_at_time(time, quantum);
            let _peers = link.num_peers();
            let phase = session_state.phase_at_time(time, quantum);

            let compensated_phase = phase + latency_comp;
            let compensated_beat = beat + latency_comp;

            // println!(
            //     "playing:{}, q:{:.2}, tempo:{:.2}, beat:{:.2}, phase:{:.2}, peers:{}",
            //     play, quantum, tempo, beat, phase, _peers
            // );

            if compensated_beat - last_beat >= 1.0 {
                last_beat = compensated_beat.floor(); // re-calibrate to full beat
                color = vis::RGB8::new_rnd();
            }

            let percentage = compensated_phase / quantum;

            // UPDATE LED DISPLAY ARRAY:
            if play {
                match vis_selected {
                    Vis::Off => {
                        leds.update_off();
                    }
                    Vis::One => {
                        leds.update_with_image(
                            gif_circular
                                .get_frame_vec_ref((compensated_phase * 4.0) as usize)
                                .unwrap_or_else(|| gif_circular.get_frame_vec_ref(0).unwrap())
                                .clone(),
                        );
                    }
                    Vis::Two => {
                        leds.update_with_image(
                            gif_rows
                                .get_frame_vec_ref((compensated_phase * 2.0) as usize)
                                .unwrap_or_else(|| gif_rows.get_frame_vec_ref(0).unwrap())
                                .clone(),
                        );
                    }
                    Vis::Three => {
                        leds.update_off();
                        leds.update_clockwise(percentage as f32, color);
                    }
                }

                if vis_numbers {
                    leds.update_with_image(
                        gif_counter
                            .get_frame_vec_ref(compensated_phase as usize)
                            .unwrap_or_else(|| gif_counter.get_frame_vec_ref(0).unwrap())
                            .clone(),
                    );
                }
            }
        });

        // GET GUI INPUT
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("rusty_chain_link")
                .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
                .auto_sized()
                .min_width(constants::WIDTH as f32 * 0.15)
                .show(egui_ctx, |ui| {
                    ui.add(
                        egui::Slider::new(&mut quantum, 2.0..=4.0)
                            .integer()
                            .text("quantum"),
                    );
                    ui.add(egui::Slider::new(&mut latency_comp, 0.0..=0.1).text("latency comp"));
                    ui.add(egui::Checkbox::new(&mut link_enabled, "link enabled?"));
                    ui.add(egui::Checkbox::new(&mut play, "start/stop?"));
                    ui.add(
                        egui::Slider::new(&mut tempo, 20.0..=999.0)
                            .integer()
                            .text("bpm"),
                    );
                    ui.add(egui::Checkbox::new(&mut vis_numbers, "numbers?"));
                    egui::ComboBox::from_label("select vis")
                        .selected_text(format!("{:?}", vis_selected))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut vis_selected,
                                Vis::Off,
                                format!("{:?}", Vis::Off),
                            );
                            ui.selectable_value(
                                &mut vis_selected,
                                Vis::One,
                                format!("{:?}", Vis::One),
                            );
                            ui.selectable_value(
                                &mut vis_selected,
                                Vis::Two,
                                format!("{:?}", Vis::Two),
                            );
                            ui.selectable_value(
                                &mut vis_selected,
                                Vis::Three,
                                format!("{:?}", Vis::Three),
                            );
                        });
                });
        });

        // UPDATE LINK WITH GUI CHANGES:
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

        // DRAW GUI:
        egui_macroquad::draw();

        // DRAW LED ARRAY:
        leds.draw_centered();

        // AWAIT NEXT FRAME:
        next_frame().await
    }
}
