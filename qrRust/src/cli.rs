// argv wrangling. tried clap, decided it was overkill for two commands.

use crate::errors::AppError;

pub enum Cmd {
    View,
    Patch { offset: usize, byte_val: u8 },
}

pub struct Args {
    pub cmd: Cmd,
    pub file: String,
}

pub fn parse_args(argv: &[String]) -> Result<Args, AppError> {
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
}
