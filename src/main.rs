extern crate hound;

use std::f32::consts::PI;
use std::i16;

mod command_line_query;
use command_line_query::ask_for_number_of_wavetables;

fn create_waveform(number_of_channels: u16) {
    let spec = hound::WavSpec {
        channels: number_of_channels,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("waveform.wav", spec).unwrap();
    for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}
fn main() {
    match ask_for_number_of_wavetables() {
        Some(v) => create_waveform(v),
        None => println!("Invalid option, exiting app.")
    }
}
