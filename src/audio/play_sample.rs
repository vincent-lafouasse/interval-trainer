use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

pub fn play_ding_in_thread() {
    std::thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open("src/assets/audio/ding.wav").unwrap());
        let source = Decoder::new(file).unwrap();

        let _ = stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_millis(300));
    });
}
