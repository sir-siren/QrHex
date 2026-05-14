#![forbid(unsafe_code)]
#![warn(unused, dead_code)]

use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
    process,
};

use thiserror::Error;

// 16 bytes per row is the universal hex dump standard. fight me.

const BYTES_PER_ROW: usize = 16;
// 10 MB cap -- if you are hex-editing something bigger, use a real tool
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

const USAGE: &str = "\
Usage:
  qrhex view  <file>
  qrhex patch <file> <offset_decimal> <byte_hex>

Examples:
  qrhex view  qr.png
  qrhex patch qr.png 24 ff
";

// every way this thing can blow up, spelled out so the user gets a real message

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not enough arguments")]
    NotEnoughArguments,

    #[error("patch requires <offset> and <byte_hex>")]
    PatchArgsMissing,

    #[error("invalid offset: expected non-negative decimal integer")]
    InvalidOffset,

    #[error("invalid byte: expected hex (e.g. ff or 0xff)")]
    InvalidHexByte,

    #[error("unknown command '{command}'")]
    UnknownCommand { command: String },

    #[error("cannot open '{path}': {reason}")]
    FileOpenFailed { path: String, reason: String },

    #[error("cannot stat '{path}': {reason}")]
    FileStatFailed { path: String, reason: String },

    #[error("file too large (max {max_bytes} bytes)")]
    FileTooLarge { max_bytes: u64 },

    #[error("failed to read '{path}': {reason}")]
    FileReadFailed { path: String, reason: String },

    #[error("failed to write temp file: {reason}")]
    TempWriteFailed { reason: String },

    #[error("failed to replace '{path}': {reason}")]
    FileReplaceFailed { path: String, reason: String },

    #[error("offset {offset} out of range (file is {file_len} bytes)")]
    OffsetOutOfRange { offset: usize, file_len: usize },

    #[error("failed to clean up temp file '{path}': {reason}")]
    TempCleanupFailed { path: String, reason: String },
}

// argv wrangling. tried clap, decided it was overkill for two commands.

enum Cmd {
    View,
    Patch { offset: usize, byte_val: u8 },
}

struct Args {
    cmd: Cmd,
    file: String,
}

fn parse_args(argv: &[String]) -> Result<Args, AppError> {
    if argv.len() < 3 {
        return Err(AppError::NotEnoughArguments);
    }

    let file = argv[2].clone();

    let cmd = match argv[1].as_str() {
        "view" => Cmd::View,

        "patch" => {
            if argv.len() < 5 {
                return Err(AppError::PatchArgsMissing);
            }
            let offset = argv[3]
                .parse::<usize>()
                .map_err(|_| AppError::InvalidOffset)?;

            let hex_str = argv[4]
                .trim_start_matches("0x")
                .trim_start_matches("0X");
            let byte_val = u8::from_str_radix(hex_str, 16)
                .map_err(|_| AppError::InvalidHexByte)?;

            Cmd::Patch { offset, byte_val }
        }

        other => return Err(AppError::UnknownCommand { command: other.to_string() }),
    };

    Ok(Args { cmd, file })
}

// the scariest part -- touching the user's files without corrupting them

fn read_file(path: &str) -> Result<Vec<u8>, AppError> {
    let mut file = File::open(path).map_err(|e| AppError::FileOpenFailed {
        path: path.to_string(),
        reason: e.to_string(),
    })?;

    let size = file
        .metadata()
        .map_err(|e| AppError::FileStatFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?
        .len();

    if size > MAX_FILE_SIZE {
        return Err(AppError::FileTooLarge { max_bytes: MAX_FILE_SIZE });
    }

    let mut data = Vec::with_capacity(size as usize);
    file.read_to_end(&mut data).map_err(|e| AppError::FileReadFailed {
        path: path.to_string(),
        reason: e.to_string(),
    })?;

    Ok(data)
}

fn write_file(path: &str, data: &[u8]) -> Result<(), AppError> {
    let original_path = Path::new(path);

    let perm = fs::metadata(path)
        .map_err(|e| AppError::FileStatFailed {
            path: path.to_string(),
            reason: e.to_string(),
        })?
        .permissions();

    // parent() returns None for bare filenames. "." is fine.
    let dir = original_path.parent().unwrap_or(Path::new("."));
    let stem = original_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("patch");
    let tmp_path = dir.join(format!(".qrhex-{stem}.tmp"));

    // closure trick -- lets us use ? for io::Result inside, then handle it outside
    let write_result = (|| -> io::Result<()> {
        let mut tmp = File::create(&tmp_path)?;
        tmp.set_permissions(perm)?;
        tmp.write_all(data)?;
        tmp.flush()?;
        Ok(())
    })();

    if let Err(e) = write_result {
        // write failed. try to nuke the half-written temp file before bailing.
        cleanup_temp_file(&tmp_path)?;
        return Err(AppError::TempWriteFailed { reason: e.to_string() });
    }

    fs::rename(&tmp_path, path).map_err(|e| {
        // rename died. try to clean up the mess. if cleanup also dies, scream.
        if let Err(cleanup_err) = cleanup_temp_file(&tmp_path) {
            eprintln!("warning: {cleanup_err}");
        }
        AppError::FileReplaceFailed {
            path: path.to_string(),
            reason: e.to_string(),
        }
    })
}

// `let _ = remove_file()` was here before. never again. errors exist for a reason.
fn cleanup_temp_file(path: &Path) -> Result<(), AppError> {
    if path.exists() {
        fs::remove_file(path).map_err(|e| AppError::TempCleanupFailed {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;
    }
    Ok(())
}

// classic hex dump -- offset | hex bytes | ascii. nothing fancy.

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

// one byte, one write. that is the whole feature.

fn patch_byte(data: &mut [u8], offset: usize, val: u8) -> Result<(), AppError> {
    if offset >= data.len() {
        return Err(AppError::OffsetOutOfRange {
            offset,
            file_len: data.len(),
        });
    }
    data[offset] = val;
    Ok(())
}

// run() does the real work. main() just catches the fallout.

fn run(argv: &[String]) -> Result<(), AppError> {
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

// if any of these break, something fundamental changed. investigate.

#[cfg(test)]
mod tests {
    use super::*;

    // argv edge cases -- every dumb thing a user could type

    #[test]
    fn parse_args_view_command_succeeds() {
        let argv = vec![
            "qrhex".to_string(),
            "view".to_string(),
            "test.bin".to_string(),
        ];
        let args = parse_args(&argv).unwrap();
        assert!(matches!(args.cmd, Cmd::View));
        assert_eq!(args.file, "test.bin");
    }

    #[test]
    fn parse_args_patch_command_succeeds() {
        let argv = vec![
            "qrhex".to_string(),
            "patch".to_string(),
            "test.bin".to_string(),
            "10".to_string(),
            "ff".to_string(),
        ];
        let args = parse_args(&argv).unwrap();
        assert!(matches!(args.cmd, Cmd::Patch { offset: 10, byte_val: 0xff }));
        assert_eq!(args.file, "test.bin");
    }

    #[test]
    fn parse_args_patch_accepts_0x_prefix() {
        let argv = vec![
            "qrhex".to_string(),
            "patch".to_string(),
            "f.bin".to_string(),
            "0".to_string(),
            "0xAB".to_string(),
        ];
        let args = parse_args(&argv).unwrap();
        assert!(matches!(args.cmd, Cmd::Patch { offset: 0, byte_val: 0xAB }));
    }

    #[test]
    fn parse_args_not_enough_args_fails() {
        let argv = vec!["qrhex".to_string()];
        let result = parse_args(&argv);
        assert!(matches!(result, Err(AppError::NotEnoughArguments)));
    }

    #[test]
    fn parse_args_patch_missing_byte_fails() {
        let argv = vec![
            "qrhex".to_string(),
            "patch".to_string(),
            "test.bin".to_string(),
            "10".to_string(),
        ];
        let result = parse_args(&argv);
        assert!(matches!(result, Err(AppError::PatchArgsMissing)));
    }

    #[test]
    fn parse_args_invalid_offset_fails() {
        let argv = vec![
            "qrhex".to_string(),
            "patch".to_string(),
            "test.bin".to_string(),
            "abc".to_string(),
            "ff".to_string(),
        ];
        let result = parse_args(&argv);
        assert!(matches!(result, Err(AppError::InvalidOffset)));
    }

    #[test]
    fn parse_args_invalid_hex_byte_fails() {
        let argv = vec![
            "qrhex".to_string(),
            "patch".to_string(),
            "test.bin".to_string(),
            "0".to_string(),
            "zz".to_string(),
        ];
        let result = parse_args(&argv);
        assert!(matches!(result, Err(AppError::InvalidHexByte)));
    }

    #[test]
    fn parse_args_unknown_command_fails() {
        let argv = vec![
            "qrhex".to_string(),
            "delete".to_string(),
            "test.bin".to_string(),
        ];
        let result = parse_args(&argv);
        assert!(matches!(result, Err(AppError::UnknownCommand { .. })));
    }

    // boundary math. off-by-one errors live here.

    #[test]
    fn patch_byte_writes_value_at_offset() {
        let mut data = vec![0x00, 0x11, 0x22, 0x33];
        patch_byte(&mut data, 2, 0xFF).unwrap();
        assert_eq!(data[2], 0xFF);
    }

    #[test]
    fn patch_byte_at_last_valid_offset_succeeds() {
        let mut data = vec![0x00; 4];
        patch_byte(&mut data, 3, 0xAA).unwrap();
        assert_eq!(data[3], 0xAA);
    }

    #[test]
    fn patch_byte_out_of_range_fails() {
        let mut data = vec![0x00; 4];
        let result = patch_byte(&mut data, 4, 0xFF);
        assert!(matches!(result, Err(AppError::OffsetOutOfRange { offset: 4, file_len: 4 })));
    }

    #[test]
    fn patch_byte_on_empty_data_fails() {
        let mut data: Vec<u8> = Vec::new();
        let result = patch_byte(&mut data, 0, 0x01);
        assert!(matches!(result, Err(AppError::OffsetOutOfRange { offset: 0, file_len: 0 })));
    }

    // the real test -- write bytes, read them back, pray they match

    #[test]
    fn read_file_nonexistent_path_fails() {
        let result = read_file("__this_file_does_not_exist_qrhex_test__");
        assert!(matches!(result, Err(AppError::FileOpenFailed { .. })));
    }

    #[test]
    fn write_and_read_round_trip_preserves_data() {
        let dir = std::env::temp_dir();
        let path = dir.join("qrhex_test_round_trip.bin");
        let path_str = path.to_str().unwrap();

        // write_file reads permissions from the original. no file = no perms = boom.
        std::fs::write(&path, b"seed").unwrap();

        let payload: Vec<u8> = (0..=255).collect();
        write_file(path_str, &payload).unwrap();

        let read_back = read_file(path_str).unwrap();
        assert_eq!(read_back, payload);

        // test cleanup -- yes this swallows the error. it is a temp file in tests. relax.
        let _ = std::fs::remove_file(&path);
    }
}