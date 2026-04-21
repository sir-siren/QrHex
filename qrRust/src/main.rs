use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    process,
};

const BYTES_PER_ROW: usize = 16;
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

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

    let file = argv[2].clone();

    let cmd = match argv[1].as_str() {
        "view" => Cmd::View,

        "patch" => {
            if argv.len() < 5 {
                return Err("patch requires <offset> and <byte_hex>".into());
            }
            let offset = argv[3]
                .parse::<usize>()
                .map_err(|_| "invalid offset: expected non-negative decimal integer".to_string())?;

            // FIX [LOW-002]: strip "0x"/"0X" prefix so both "ff" and "0xff" are accepted
            let hex_str = argv[4]
                .trim_start_matches("0x")
                .trim_start_matches("0X");
            let byte_val = u8::from_str_radix(hex_str, 16)
                .map_err(|_| "invalid byte: expected hex (e.g. ff or 0xff)".to_string())?;

            Cmd::Patch { offset, byte_val }
        }

        other => return Err(format!("unknown command '{other}'")),
    };

    Ok(Args { cmd, file })
}

// FIX [HIGH-002]: open handle first, check metadata.len() before allocating,
// then read. The old code called fs::read() which allocated the full Vec before
// the size guard fired — an OOM DoS vector on large files.
fn read_file(path: &str) -> Result<Vec<u8>, String> {
    let mut file =
        File::open(path).map_err(|e| format!("cannot open '{path}': {e}"))?;

    let size = file
        .metadata()
        .map_err(|e| format!("cannot stat '{path}': {e}"))?
        .len();

    if size > MAX_FILE_SIZE {
        return Err(format!(
            "file too large (max {} bytes)",
            MAX_FILE_SIZE
        ));
    }

    let mut data = Vec::with_capacity(size as usize);
    file.read_to_end(&mut data)
        .map_err(|e| format!("failed to read '{path}': {e}"))?;

    Ok(data)
}

// FIX [HIGH-001, MED-002]: write atomically via temp file + rename.
// Preserves original file permissions. A crash mid-write leaves the original
// intact; the temp file is cleaned up on next run or by the OS.
fn write_file(path: &str, data: &[u8]) -> Result<(), String> {
    let original_path = Path::new(path);

    // Preserve original permissions before touching anything.
    let perm = fs::metadata(path)
        .map_err(|e| format!("cannot stat '{path}': {e}"))?
        .permissions();

    let dir = original_path.parent().unwrap_or(Path::new("."));
    let tmp_path = dir.join(format!(
        ".qrhex-{}.tmp",
        original_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("patch")
    ));

    // Write to temp, clean it up on any failure.
    let write_result = (|| -> io::Result<()> {
        let mut tmp = File::create(&tmp_path)?;
        tmp.set_permissions(perm)?;
        tmp.write_all(data)?;
        tmp.flush()?;
        Ok(())
    })();

    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp_path);
        return Err(format!("failed to write temp file: {e}"));
    }

    // Atomic on POSIX and Windows (same filesystem).
    fs::rename(&tmp_path, path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        format!("failed to replace '{path}': {e}")
    })
}

fn print_hex_dump(data: &[u8]) {
    for (chunk_idx, row) in data.chunks(BYTES_PER_ROW).enumerate() {
        let row_start = chunk_idx * BYTES_PER_ROW;

        print!("{row_start:08x}  ");

        for (i, b) in row.iter().enumerate() {
            if i == 8 {
                print!(" ");
            }
            print!("{b:02x} ");
        }

        let pad = BYTES_PER_ROW - row.len();
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
    let args = parse_args(argv)?;
    let mut data = read_file(&args.file)?;

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