<p align="center">
    <img src="./public/QrHex.png" alt="QrHex" width="120" />
</p>

<h1 align="center">QrHex</h1>

<p align="center">
    <strong>
        A small command line tool to view and edit binary files byte by byte. Made for looking inside QR code PNG files, but works on any binary file.
    </strong>
</p>

<p align="center">
    <a href="https://ziglang.org/download/">
        <img alt="Zig" src="https://img.shields.io/badge/Zig-0.15.2-F7D080?style=flat&logo=zig&logoColor=F7D080" />
    </a>
    <a href="https://www.rust-lang.org">
        <img alt="Rust" src="https://img.shields.io/badge/Rust-edition%202024-DEA584?style=flat&logo=rust&logoColor=FF9170" />
    </a>
    <a>
        <img alt="Zero dependencies" src="https://img.shields.io/badge/dependencies-none-B5EAD7?style=flat" />
    </a>
    <a href="./LICENSE">
        <img alt="License" src="https://img.shields.io/badge/license-MIT-AEC6CF?style=flat" />
    </a>

</p>

## 📖 Overview

**QrHex** has two commands: `view` prints any binary file as a hex table, and `patch` overwrites one byte at any position. Both implementations use stdlib only with no external libraries.

Two versions of the same tool:

| Version    | Folder                   | Language                       |
| :--------- | :----------------------- | :----------------------------- |
| **qrZig**  | [`./qrZig/`](./qrZig/)   | Zig 0.15.2, stdlib only        |
| **qrRust** | [`./qrRust/`](./qrRust/) | Rust edition 2024, stdlib only |

## 📂 Repository Structure

```bash
 .
├──  public
│   └──  QrHex.png
├──  qrRust
│   ├── 󰣞  src
│   │   └──  main.rs
│   ├──  Cargo.lock
│   └──  Cargo.toml
├──  qrZig
│   ├── 󰣞  src
│   │   └──  main.zig
│   ├──  build.zig
│   └──  build.zig.zon
├──  CITATION.cff
├──  CODE_OF_CONDUCT.md
├── 󰡯 CODEOWNERS
├──  LICENSE
└── 󰂺 README.md
```

## ⌨️ Commands

```sh
qrhex view  <file>
qrhex patch <file> <offset> <byte>
```

### view

Prints the file as a hex table. 16 bytes per row with a text preview on the right.

```bash
00000000  89 50 4e 47 0d 0a 1a 0a  00 00 00 0d 49 48 44 52  |.PNG........IHDR|
00000010  00 00 00 21 00 00 00 21  08 02 00 00 00 49 b8 d6  |...!...!.....I..|
00000020  65 00 00 00 09 70 48 59  73 00 00 0e c4 00 00 0e  |e....pHYs.......|

1024 bytes
```

### patch

Changes one byte at a given position and saves the file.

```bash
patched: offset 0x00000018 (24) -> 0x00
```

### Command reference

| Command | Arguments                | Description                          |
| :------ | :----------------------- | :----------------------------------- |
| `view`  | `<file>`                 | Print a hex dump of the file         |
| `patch` | `<file> <offset> <byte>` | Write one byte at the given position |

- `offset` is a **decimal** number — example: `24`
- `byte` is a **hex** value — example: `ff`

> [!NOTE]
> Both versions load the whole file into memory. Files larger than **10 MB** will be rejected.

## 🚀 Getting Started

### Prerequisites

You only need one of the two. Pick whichever language you prefer.

| Tool | Version      | Install                                               |
| :--- | :----------- | :---------------------------------------------------- |
| Zig  | `0.15.2+`    | [ziglang.org/download](https://ziglang.org/download/) |
| Rust | edition 2024 | [rustup.rs](https://rustup.rs)                        |

## ⚡ Zig Version

### Verify install

```sh
zig version
# expected: 0.15.2
```

### Setup

```sh
cd qrZig
```

### Build

```sh
# debug
zig build

# optimized
zig build -Doptimize=ReleaseSafe
```

Output binary: `zig-out/bin/qrhex`

### Run

```sh
# through the build system
zig build run -- view qr.png
zig build run -- patch qr.png 24 ff

# or run the binary directly
./zig-out/bin/qrhex view qr.png
./zig-out/bin/qrhex patch qr.png 24 ff
```

## 🦀 Rust Version

### Verify install

```sh
rustc --version
cargo --version
```

### Setup

```sh
cd qrRust
```

### Build

```sh
# debug
cargo build

# optimized
cargo build --release
```

Output binaries:

| Mode    | Path                   |
| :------ | :--------------------- |
| Debug   | `target/debug/qrhex`   |
| Release | `target/release/qrhex` |

### Run

```sh
# through cargo
cargo run -- view qr.png
cargo run -- patch qr.png 24 ff

# optimized
cargo run --release -- view qr.png

# or run the binary directly
./target/debug/qrhex view qr.png
./target/debug/qrhex patch qr.png 24 ff
```

## 🔬 How QR PNG Files Are Structured

A QR code saved as a PNG starts with a fixed header. You can use `patch` safely on the bytes listed here.

| Offset | Size    | Value         | What it is               |
| :----- | :------ | :------------ | :----------------------- |
| `0`    | 4 bytes | `89 50 4e 47` | PNG file signature       |
| `8`    | 4 bytes | `00 00 00 0d` | Length of the IHDR chunk |
| `12`   | 4 bytes | `49 48 44 52` | The text `IHDR`          |
| `16`   | 4 bytes |               | Image width              |
| `20`   | 4 bytes |               | Image height             |

> [!WARNING]
> QR pixel data is stored compressed inside `IDAT` chunks. Patching bytes in that region will corrupt the file. Only use `patch` on header bytes like the ones in the table above.

To change the actual content of a QR code, use these tools instead:

1. Decode the QR with [`zbarimg`](https://zbar.sourceforge.net/) to read the text inside
2. Edit the text
3. Re-encode it into a new QR with [`qrencode`](https://fukuchi.org/works/qrencode/)

## 📜 License

This project is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

<p align="center">
  <sub>Built with 🦀 and ⚡</sub>
</p>
