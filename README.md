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
        <img alt="Rust" src="https://img.shields.io/badge/Rust-1.95.0-DEA584?style=flat&logo=rust&logoColor=FF9170" />
    </a>
    <a href="https://go.dev">
        <img alt="Go" src="https://img.shields.io/badge/Go-1.26.2-00ADD8?style=flat&logo=go&logoColor=00ADD8" />
    </a>
    <a>
        <img alt="Zero dependencies" src="https://img.shields.io/badge/dependencies-none-B5EAD7?style=flat" />
    </a>
    <a href="./LICENSE">
        <img alt="License" src="https://img.shields.io/badge/license-MIT-AEC6CF?style=flat" />
    </a>
</p>

## 📖 Overview

**QrHex** has two commands: `view` prints any binary file as a hex table, and `patch` overwrites one byte at any position. All implementations use stdlib only with no external libraries.

Three versions of the same tool:

| Version    | Folder                   | Language                 |
| :--------- | :----------------------- | :----------------------- |
| **qrZig**  | [`./qrZig/`](./qrZig/)   | Zig 0.15.2, stdlib only  |
| **qrRust** | [`./qrRust/`](./qrRust/) | Rust 1.95.0, stdlib only |
| **qrGo**   | [`./qrGo/`](./qrGo/)     | Go 1.26.2, stdlib only   |

## 📂 Repository Structure

```bash
.
├── public
│   ├── cat.png
│   └── QrHex.png
├── qrGo
│   ├── go.mod
│   ├── main.go
│   └── mise.toml
├── qrRust
│   ├── src
│   │   └── main.rs
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── mise.toml
├── qrZig
│   ├── src
│   │   └── main.zig
│   ├── build.zig
│   ├── build.zig.zon
│   └── mise.toml
├── CITATION.cff
├── CODE_OF_CONDUCT.md
├── CODEOWNERS
├── LICENSE
└── README.md
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
> All versions load the whole file into memory. Files larger than **10 MB** will be rejected.

## 🚀 Getting Started

### Install mise

Each project folder has a `mise.toml` that pins the exact toolchain version. Install [mise](https://mise.jdx.dev) once and it handles the rest.

**macOS / Linux**

```sh
curl https://mise.run | sh
```

**Homebrew**

```sh
brew install mise
```

**Windows**

```powershell
winget install jdx.mise
```

**Ubuntu / Debian (apt)**

```sh
sudo apt update -y && sudo apt install -y curl
sudo install -dm 755 /etc/apt/keyrings
curl -fsSL https://mise.jdx.dev/gpg-key.pub | sudo tee /etc/apt/keyrings/mise-archive-keyring.asc > /dev/null
echo "deb [signed-by=/etc/apt/keyrings/mise-archive-keyring.asc] https://mise.jdx.dev/deb stable main" | sudo tee /etc/apt/sources.list.d/mise.list
sudo apt update -y && sudo apt install -y mise
```

### Activate mise in your shell

Skip this step if you installed via Homebrew — it activates automatically.

```sh
# bash
echo 'eval "$(mise activate bash)"' >> ~/.bashrc

# zsh
echo 'eval "$(mise activate zsh)"' >> ~/.zshrc

# fish
echo 'mise activate fish | source' >> ~/.config/fish/config.fish

# PowerShell
echo '(&mise activate pwsh) | Out-String | Invoke-Expression' >> $HOME\Documents\PowerShell\Microsoft.PowerShell_profile.ps1
```

Restart your shell, then verify everything is set up:

```sh
mise doctor
```

### Prerequisites

You only need one of the three. Pick whichever language you prefer — mise will install it for you.

| Tool | Version  | mise name |
| :--- | :------- | :-------- |
| Zig  | `0.15.2` | `zig`     |
| Rust | `1.95.0` | `rust`    |
| Go   | `1.26.2` | `go`      |

## ⚡ Zig Version

### Setup

```sh
cd qrZig

# install the pinned Zig version
mise install
```

### Verify install

```sh
zig version
# expected: 0.15.2
```

### Tasks

```bash
mise run build
./zig-out/bin/qrhex view ../public/cat.png
```

### Build manually

```sh
zig build
```

Output binary: `zig-out/bin/qrhex`

### Run

```sh
# through the build system
zig build run -- view ../public/cat.png
zig build run -- patch ../public/cat.png 24 ff

# or run the binary directly
./zig-out/bin/qrhex view ../public/cat.png
./zig-out/bin/qrhex patch ../public/cat.png 24 ff
```

## 🦀 Rust Version

### Setup

```sh
cd qrRust

# install the pinned Rust version
mise install
```

### Verify install

```sh
rustc --version
cargo --version
# expected: rustc 1.95.0
```

### Tasks

```sh
mise run dev      # debug build
mise run build    # optimized build (--release)
mise run preview  # build and run view on cat.png
```

### Build manually

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
cargo run -- view ../public/cat.png
cargo run -- patch ../public/cat.png 24 ff

# or run the binary directly
./target/release/qrhex view ../public/cat.png
./target/release/qrhex patch ../public/cat.png 24 ff
```

## 🐹 Go Version

### Setup

```sh
cd qrGo

# install the pinned Go version
mise install
```

### Verify install

```sh
go version
# expected: go1.26.2
```

### Tasks

```sh
mise run dev      # debug build
mise run build    # optimized build (-ldflags="-s -w")
mise run preview  # build and run view on cat.png
```

### Build manually

```sh
# debug
go build -o qrhex .

# optimized (trimmed binary, no debug info)
go build -ldflags="-s -w" -o qrhex .
```

Output binary: `./qrhex`

### Run

```sh
# through go run (no build step)
go run . view ../public/cat.png
go run . patch ../public/cat.png 24 ff

# or run the binary directly
./qrhex view ../public/cat.png
./qrhex patch ../public/cat.png 24 ff
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
  <sub>Built with 🦀 ⚡ and 🐹</sub>
</p>
