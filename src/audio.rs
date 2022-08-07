use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rodio::*;
use rodio::{source::Source, Decoder, OutputStream};

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
