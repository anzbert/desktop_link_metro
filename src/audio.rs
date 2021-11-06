use rodio::*;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;

// use cpal::traits::HostTrait;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;

pub fn audio_thread_cpal() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = supported_config.into();

    fn write_to_stream() {}

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {},
        move |err| println!("error: {:?}", err),
    );

    // let (audio_tx, audio_rx): (SyncSender<bool>, Receiver<bool>) = sync_channel(1);

    // let _audio_handle = thread::spawn(move || {
    //     // Get a output stream handle to the default physical sound device
    //     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //     let sink = Sink::try_new(&stream_handle).unwrap();
    //     let file = BufReader::new(File::open("snd/met_mech.wav").unwrap());

    //     for message in audio_rx {
    //         match message {
    //             true => {
    //                 let source = Decoder::new(file).unwrap();

    //                 sink.append(source);
    //                 sink.sleep_until_end();
    //                 // stream_handle.play_raw(source.convert_samples()).unwrap();
    //                 // std::thread::sleep(std::time::Duration::from_secs(5));
    //             }
    //             false => {}
    //         }
    //     }
    // });
    // audio_tx
}
#[derive(Clone)]
struct Sound {
    sound: Cursor<Vec<u8>>,
}
impl Sound {
    fn new(path: &str) -> Self {
        Self {
            sound: Cursor::new(std::fs::read(path).unwrap()),
        }
    }
    fn play(self, stream_handle: &OutputStreamHandle) {
        let source = Decoder::new(self.sound).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();
    }
}

pub fn metro_audio_init() -> Sender<u32> {
    let (audio_tx, audio_rx): (Sender<u32>, Receiver<u32>) = std::sync::mpsc::channel();

    let _audio_handle = thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let sound_0 = Sound::new("snd/met_mech.wav");
        let sound_1 = Sound::new("snd/met_elec.wav");

        for message in audio_rx {
            match message {
                0 => sound_0.clone().play(&stream_handle),
                1 => sound_1.clone().play(&stream_handle),
                _ => println!("Sound not available"),
            };
        }
    });

    audio_tx
}
