// classic hex dump -- offset | hex bytes | ascii. nothing fancy.

// 16 bytes per row is the universal hex dump standard. fight me.
const BYTES_PER_ROW: usize = 16;

pub fn print_hex_dump(data: &[u8]) {
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
