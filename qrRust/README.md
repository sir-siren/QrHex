# qrhex (Rust)

A minimal CLI hex editor for QR code files (or any binary file).

```
qrhex view  <file>
qrhex patch <file> <offset_decimal> <byte_hex>
```

---

## Requirements

- [Rust](https://rustup.rs) (project uses edition 2024; tested on 1.94.1)

Verify:

```sh
rustc --version
cargo --version
```

---

## Dev setup

```sh
git clone <your-repo>
cd qrRust
```

Project layout:

```bash
 . qrRust
├──  src
│  └──  main.rs
├──  Cargo.lock
├──  Cargo.toml
└──  README.md
```

---

## Build

### Debug

```sh
cargo build
# binary -> target/debug/qrhex
```

### Release

```sh
cargo build --release
# binary -> target/release/qrhex
```

---

## Preview / run without installing

```sh
# View hex dump of a QR image
cargo run -- view qr.png

# Patch byte at decimal offset 24 to 0xff
cargo run -- patch qr.png 24 ff

# Release mode run
cargo run --release -- view qr.png
```

Or run the binary directly after building:

```sh
./target/debug/qrhex view qr.png
./target/debug/qrhex patch qr.png 24 ff
```

---

## Example output

```
$ qrhex view qr.png

00000000  89 50 4e 47 0d 0a 1a 0a  00 00 00 0d 49 48 44 52  |.PNG........IHDR|
00000010  00 00 00 21 00 00 00 21  08 02 00 00 00 49 b8 d6  |...!...!.....I..|
00000020  65 00 00 00 09 70 48 59  73 00 00 0e c4 00 00 0e  |e....pHYs.......|

1024 bytes
```

```
$ qrhex patch qr.png 24 00
patched: offset 0x00000018 (24) -> 0x00
```

---

## How QR hex editing works

A QR saved as PNG has this layout at the start:

| Offset | Bytes         | Meaning      |
| ------ | ------------- | ------------ |
| 0      | `89 50 4e 47` | PNG magic    |
| 8      | `00 00 00 0d` | IHDR length  |
| 12     | `49 48 44 52` | `IHDR` ASCII |
| 16–19  | 4 bytes       | image width  |
| 20–23  | 4 bytes       | image height |

> ⚠️ QR pixel data in PNG is zlib-compressed inside IDAT chunks.
> Patching raw pixel bytes directly won't work — decompress first,
> modify, then recompress. Use `patch` for header fields or metadata.
