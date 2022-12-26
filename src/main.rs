use std::fs::OpenOptions;
use std::io::Write;
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Path to FIFO file
    #[arg(short, long, default_value_t = String::from("/tmp/passwd-daemon"))]
    path: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let mut fifo = match OpenOptions::new().read(false).write(true).create(false).open(&*args.path) {
        Ok(fifo) => fifo,
        Err(e) => {
            eprintln!("Failed to open '{}' with write permissions: {}", &args.path, e);
            return ExitCode::FAILURE;
        }
    };
    // OpenWindow
    match fifo.write(&[0]) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Failed to write to '{}': {:?}", &args.path, e);
            ExitCode::FAILURE
        }
    }
}
