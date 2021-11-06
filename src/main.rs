mod audio;
mod constants;
mod input;
mod vis;

use gif2json::RgbaImageData;
use macroquad::prelude::*;

use egui_macroquad::*;

// MQ WINDOW CONFIG:
fn window_conf() -> Conf {
    Conf {
        window_title: "rusty_link".to_owned(),
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
    // INIT SOUND
    let audio_tx = audio::metro_audio_init();
    let mut sound_on = true;

    // Init LINK:
    let mut link = ableton_link::Link::new(120.0);
    link.enable(true);

    let mut link_enabled = true;
    link.enable_start_stop_sync(true);

    let clock = link.clock();
    let mut quantum = 4.0;

    let mut tempo: f64 = 0.0;
    let mut last_tempo: f64 = 0.0;

    let mut last_beat: f64 = 0.0;

    let mut latency_comp = 0.0;

    link.with_app_session_state(|ss| {
        tempo = ss.tempo();
        last_tempo = tempo;
    });

    // Init VISUALS:
    let mut vis_on = true;

    #[derive(PartialEq, Debug)]
    enum Vis {
        Off,
        One,
        Two,
        Three,
        Four,
    }
    let mut vis_selected = Vis::One;

    let mut vis_numbers = true;

    let mut leds = vis::Leds::new();
    let mut new_color_on_beat = vis::RGB8::new_rnd();

    let gif_counter =
        RgbaImageData::new_from_bytes(include_bytes!("../img/counter_alpha.gif")).unwrap();
    let gif_clock = RgbaImageData::new_from_bytes(include_bytes!("../img/clock.gif")).unwrap();
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
            tempo = session_state.tempo();
            let time = clock.micros();
            let beat = session_state.beat_at_time(time, quantum);
            let phase = session_state.phase_at_time(time, quantum);

            let _peers = link.num_peers();
            let _play = session_state.is_playing();

            // latency compensation - this idea doesnt work too well yet
            let compensated_phase = phase + latency_comp;
            let compensated_beat = beat + latency_comp;

            // println!(
            //     "playing:{}, q:{:.2}, tempo:{:.2}, beat:{:.2}, phase:{:.2}, peers:{}",
            //     _play, quantum, tempo, beat, phase, _peers
            // );

            // ROUTINE (on every full beat):
            if compensated_beat - last_beat >= 1.0 {
                last_beat = compensated_beat.floor(); // re-calibrate to full beat

                new_color_on_beat = vis::RGB8::new_rnd(); // change this color value every beat

                if sound_on {
                    // play sound with emphasis on the 1
                    match phase.floor() as i32 {
                        0 => audio_tx.send(1).unwrap(),
                        _ => audio_tx.send(0).unwrap(),
                    }
                }
            }

            // UPDATE LED DISPLAY ARRAY (every frame):
            if vis_on {
                let phase_percentage = compensated_phase / quantum;
                match vis_selected {
                    Vis::Off => {
                        leds.update_off();
                    }
                    Vis::One => {
                        leds.update_with_image(
                            gif_circular
                                .get_frame_vec_ref((phase_percentage * 16.0) as usize)
                                .unwrap_or_else(|| gif_circular.get_frame_vec_ref(0).unwrap())
                                .clone(),
                        );
                    }
                    Vis::Two => {
                        leds.update_with_image(
                            gif_rows
                                .get_frame_vec_ref((phase_percentage * 8.0) as usize)
                                .unwrap_or_else(|| gif_rows.get_frame_vec_ref(0).unwrap())
                                .clone(),
                        );
                    }
                    Vis::Three => {
                        leds.update_off();
                        leds.update_clockwise(phase_percentage as f32, new_color_on_beat);
                    }
                    Vis::Four => {
                        leds.update_with_image(
                            gif_clock
                                .get_frame_vec_ref((phase_percentage * 4.0) as usize)
                                .unwrap_or_else(|| gif_clock.get_frame_vec_ref(0).unwrap())
                                .clone(),
                        );
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

        // DEFINE GUI LAYOUT
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("rusty_link")
                .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
                .collapsible(false)
                .auto_sized()
                .min_width(constants::WIDTH as f32 * 0.15)
                .show(egui_ctx, |ui| {
                    ui.add(egui::Checkbox::new(&mut link_enabled, "link enabled?"));
                    ui.add(
                        egui::Slider::new(&mut quantum, 2.0..=4.0)
                            .integer()
                            .text("quantum"),
                    );
                    ui.add(egui::Slider::new(&mut latency_comp, 0.0..=0.1).text("latency comp"));
                    ui.add(
                        egui::Slider::new(&mut tempo, 20.0..=200.0)
                            .integer()
                            .text("bpm"),
                    );
                    ui.add(egui::Checkbox::new(&mut sound_on, "sound?"));
                    ui.add(egui::Checkbox::new(&mut vis_on, "update vis?"));
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
                            ui.selectable_value(
                                &mut vis_selected,
                                Vis::Four,
                                format!("{:?}", Vis::Four),
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
            last_tempo = tempo;
        }

        // DRAW LED ARRAY:
        leds.draw_centered();

        // DRAW GUI:
        egui_macroquad::draw();

        // MACROQUAD ROUTINE: AWAIT NEXT FRAME:
        next_frame().await
    }
}
