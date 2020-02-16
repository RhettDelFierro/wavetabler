use std::io;

pub fn ask_for_number_of_wavetables() -> String {
    println!("How Stereo or Mono?");
    println!("Mono: 1");
    println!("Stereo: 2");

    let mut channels = String::new();
    io::stdin().read_line(&mut channels);
    channels
}