// Simple executable that reads a file, parses it, and pretty-prints it.
extern mod midi;
use midi::{parse_file, pretty_print};

fn main() {
    match parse_file("la_overworld.mid") {
        Some(file) => {
            pretty_print(file);
        }
        None => {
            println!("Didn't work.");
        }
    }
}
