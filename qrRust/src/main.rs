use std::{env, fs, process};

const BYTES_PER_ROW: usize = 16;
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

const USAGE: &str = "\
Usage:
  qrhex view  <file>
  qrhex patch <file> <offset_decimal> <byte_hex>

Examples:
  qrhex view  qr.png
  qrhex patch qr.png 24 ff
";

enum Cmd {
    View,
    Patch { offset: usize, byte_val: u8 },
}

struct Args {
    cmd: Cmd,
    file: String,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    if argv.len() < 3 {
        return Err("not enough arguments".into());
    }

    let file: String = argv[2].clone();

    let cmd: Cmd = match argv[1].as_str() {
        "view" => Cmd::View,

        "patch" => {
            if argv.len() < 5 {
                return Err("patch requires <offset> and <byte_hex>".into());
            }
            let offset: usize = argv[3]
                .parse::<usize>()
                .map_err(|_| "invalid offset: expected decimal integer".to_string())?;
            let byte_val = u8::from_str_radix(&argv[4], 16)
                .map_err(|_| "invalid byte: expected hex (e.g. ff)".to_string())?;
            Cmd::Patch { offset, byte_val }
        }

        other => return Err(format!("unknown command '{other}'")),
    };

    Ok(Args { cmd, file })
}

fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let data: Vec<u8> =
        fs::read(path).map_err(|e: std::io::Error| format!("failed to read '{path}': {e}"))?;
    if data.len() > MAX_FILE_SIZE {
        return Err(format!("file too large (max {MAX_FILE_SIZE} bytes)"));
    }
    Ok(data)
}

fn write_file(path: &str, data: &[u8]) -> Result<(), String> {
    fs::write(path, data).map_err(|e: std::io::Error| format!("failed to write '{path}': {e}"))
}

fn print_hex_dump(data: &[u8]) {
    for (chunk_idx, row) in data.chunks(BYTES_PER_ROW).enumerate() {
        let row_start: usize = chunk_idx * BYTES_PER_ROW;

        print!("{row_start:08x}  ");

        for (i, b) in row.iter().enumerate() {
            if i == 8 {
                print!(" ");
            }
            print!("{b:02x} ");
        }

        let pad: usize = BYTES_PER_ROW - row.len();
        if pad > 0 {
            if row.len() <= 8 {
                print!(" ");
            }
            print!("{}", "   ".repeat(pad));
        }

        print!(" |");
        for b in row {
            let ch = if b.is_ascii_graphic() || *b == b' ' {
                *b as char
            } else {
                '.'
            };
            print!("{ch}");
        }
        println!("|");
    }

    println!("\n{} bytes", data.len());
}

fn patch_byte(data: &mut [u8], offset: usize, val: u8) -> Result<(), String> {
    if offset >= data.len() {
        return Err(format!(
            "offset {offset} out of range (file is {} bytes)",
            data.len()
        ));
    }
    data[offset] = val;
    Ok(())
}

fn run(argv: &[String]) -> Result<(), String> {
    let args: Args = parse_args(argv)?;
    let mut data: Vec<u8> = read_file(&args.file)?;

    match args.cmd {
        Cmd::View => print_hex_dump(&data),

        Cmd::Patch { offset, byte_val } => {
            patch_byte(&mut data, offset, byte_val)?;
            write_file(&args.file, &data)?;
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
