use std::{env, fs, time::Instant};

use wat_to_wasm::compile;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match &args.as_slice() {
        [_, name, ..] => name,
        [_] => "./demo.wat",
        [] => unreachable!(),
    };

    let source = if let Ok(file_contents) = read_file(file_path) {
        println!("Compiling file {}", file_path);
        file_contents
    } else {
        eprintln!("Failed to open file");
        return;
    };

    let start_time = Instant::now();

    match compile(source) {
        Ok(_) => {
            println!(
                "Compilation successful in {}μs",
                start_time.elapsed().as_micros()
            )
        }
        Err(_) => eprintln!(
            "Compilation failed in {}μs",
            start_time.elapsed().as_micros()
        ),
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
