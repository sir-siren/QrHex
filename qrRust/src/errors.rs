// every way this thing can blow up, spelled out so the user gets a real message

use thiserror::Error;

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
