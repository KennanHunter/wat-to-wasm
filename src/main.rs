use std::{env, fs, time::Instant};

use wat_to_wasm::convert;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.len() {
        2.. => args.get(2).unwrap(),
        1 => "./demo.wat",
        0 => unreachable!(),
    };

    let source = if let Ok(file_contents) = read_file(file_path) {
        file_contents
    } else {
        eprintln!("Failed to open file");
        return;
    };

    let start_time = Instant::now();

    match convert(source) {
        Ok(_) => {
            println!(
                "Conversion successful in {}μs",
                start_time.elapsed().as_micros()
            )
        }
        Err(_) => todo!(),
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
