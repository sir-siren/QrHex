// one byte, one write. that is the whole feature.

use crate::errors::AppError;

pub fn patch_byte(data: &mut [u8], offset: usize, val: u8) -> Result<(), AppError> {
    if offset >= data.len() {
        return Err(AppError::OffsetOutOfRange {
            offset,
            file_len: data.len(),
        });
    }
    data[offset] = val;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
