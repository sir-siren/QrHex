#![forbid(unsafe_code)]
#![warn(unused, dead_code)]

mod cli;
mod errors;
mod file_io;
mod hex_dump;
mod patch;

use std::{env, process};

use cli::Cmd;
use errors::AppError;

const USAGE: &str = "\
Usage:
  qrhex view  <file>
  qrhex patch <file> <offset_decimal> <byte_hex>

Examples:
  qrhex view  qr.png
  qrhex patch qr.png 24 ff
";

// run() does the real work. main() just catches the fallout.

fn run(argv: &[String]) -> Result<(), AppError> {
    let args = cli::parse_args(argv)?;
    let mut data = file_io::read_file(&args.file)?;

    match args.cmd {
        Cmd::View => hex_dump::print_hex_dump(&data),

        Cmd::Patch { offset, byte_val } => {
            patch::patch_byte(&mut data, offset, byte_val)?;
            file_io::write_file(&args.file, &data)?;
            println!("patched: offset 0x{offset:08x} ({offset}) -> 0x{byte_val:02x}");
        }
    }

    Ok(())
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if let Err(e) = run(&argv) {
        eprintln!("error: {e}");
        eprintln!("{USAGE}");
        process::exit(1);
    }
}