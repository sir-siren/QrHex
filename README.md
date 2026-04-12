# qrhex

A minimal CLI hex editor for QR code files.

```
qrhex view  <file>
qrhex patch <file> <offset_decimal> <byte_hex>
```

---

## Requirements

- [Zig 0.15.0](https://ziglang.org/download/) - install it and make sure `zig` is on your PATH.

Verify:

```sh
zig version
# 0.15.0
```

---

## Dev setup

```sh
git clone <your-repo>
cd qrZig
```

Project layout:

```bash
 . qrzig
├── 󰣞  src
│   └──  main.zig
├──  build.zig
├──  build.zig.zon
└── 󰂺 README.md
```

---

## Build

### Debug build

```sh
zig build
```

Binary lands at `zig-out/bin/qrhex`.

### Release build

```sh
zig build -Doptimize=ReleaseSafe
```

---

## Preview / run without installing

```sh
# View hex dump of a QR image
zig build run -- view qr.png

# Patch byte at decimal offset 24 to 0xff
zig build run -- patch qr.png 24 ff
```

Or run the binary directly after building:

```sh
./zig-out/bin/qrhex view qr.png
./zig-out/bin/qrhex patch qr.png 24 ff
```

---

## Example output

```bash
$ qrhex view qr.png

00000000  89 50 4e 47 0d 0a 1a 0a  00 00 00 0d 49 48 44 52  |.PNG........IHDR|
00000010  00 00 00 21 00 00 00 21  08 02 00 00 00 49 b8 d6  |...!...!.....I..|
00000020  65 00 00 00 09 70 48 59  73 00 00 0e c4 00 00 0e  |e....pHYs.......|
...

1024 bytes
```

```bash
$ qrhex patch qr.png 24 00
patched: offset 0x00000018 (24) -> 0x00
```

---

## How QR hex editing works

A QR saved as PNG has this layout at the start:

| Offset | Bytes         | Meaning           |
| ------ | ------------- | ----------------- |
| 0      | `89 50 4e 47` | PNG magic header  |
| 8      | `00 00 00 0d` | IHDR chunk length |
| 12     | `49 48 44 52` | `IHDR` in ASCII   |
| 16–19  | 4 bytes       | image width       |
| 20–23  | 4 bytes       | image height      |

> WARN: ⚠️ QR pixel data in PNG is zlib-compressed inside IDAT chunks.
> Patching raw pixel bytes directly won't work — you'd need to decompress,
> modify the pixel matrix, then recompress. Use `patch` for header fields
> or metadata bytes, not pixel data.

For patching actual QR modules (the black/white squares), the correct approach is:

1. Decode the QR → extract payload
2. Modify the payload
3. Re-encode → new QR

Tools: `zbarimg` (decode), `qrencode` (encode).
