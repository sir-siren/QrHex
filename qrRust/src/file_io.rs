// the scariest part -- touching the user's files without corrupting them

use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use crate::errors::AppError;

// 10 MB cap -- if you are hex-editing something bigger, use a real tool
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

pub fn read_file(path: &str) -> Result<Vec<u8>, AppError> {
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

pub fn write_file(path: &str, data: &[u8]) -> Result<(), AppError> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
