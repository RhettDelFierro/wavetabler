use std::io;

pub fn ask_for_number_of_wavetables() -> Option<u8> {
    println!("How Stereo or Mono?");
    println!("Mono: 1");
    println!("Stereo: 2");
    let mut channels = String::new();
    io::stdin()
        .read_line(&mut channels)
        .expect("No value was entered");

    match channels.parse() {
        Ok(v) => {
            match v {
                1 | 2 => Some(v),
                _   => {
                    println!("Please select stereo or mono");
                    return None;
                }
            }
        },
        Err(err) => {
            println!("Please enter a value");
            return None;
        }
    }
}