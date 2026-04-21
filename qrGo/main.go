package main

import (
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"unicode"
)

const (
	bytesPerRow = 16
	maxFileSize = 10 * 1024 * 1024

	usage = `Usage:
  qrhex view  <file>
  qrhex patch <file> <offset_decimal> <byte_hex>

Examples:
  qrhex view  qr.png
  qrhex patch qr.png 24 ff
`
)

type cmd string

const (
	cmdView  cmd = "view"
	cmdPatch cmd = "patch"
)

type args struct {
	command  cmd
	filePath string
	offset   int
	byteVal  byte
}

func parseArgs(argv []string) (args, error) {
	if len(argv) < 3 {
		return args{}, errors.New("not enough arguments")
	}

	switch cmd(argv[1]) {
	case cmdView:
		return args{command: cmdView, filePath: argv[2]}, nil

	case cmdPatch:
		if len(argv) < 5 {
			return args{}, errors.New("patch requires <offset> and <byte_hex>")
		}

		offset, err := strconv.Atoi(argv[3])

		if err != nil || offset < 0 {
			return args{}, fmt.Errorf("invalid offset %q: expected non-negative decimal integer", argv[3])
		}

		hexStr := strings.TrimPrefix(strings.TrimPrefix(argv[4], "0x"), "0X")
		decoded, err := hex.DecodeString(hexStr)
		if err != nil || len(decoded) != 1 {
			return args{}, fmt.Errorf("invalid byte %q: expected hex value e.g. ff", argv[4])
		}

		return args{
			command:  cmdPatch,
			filePath: argv[2],
			offset:   offset,
			byteVal:  decoded[0],
		}, nil

	default:
		return args{}, fmt.Errorf("unknown command %q", argv[1])
	}
}

func readFile(path string) ([]byte, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("cannot open %q: %w", path, err)
	}
	defer f.Close()

	info, err := f.Stat()
	if err != nil {
		return nil, fmt.Errorf("cannot stat %q: %w", path, err)
	}
	if info.Size() > maxFileSize {
		return nil, fmt.Errorf("file too large (max %d bytes)", maxFileSize)
	}

	data, err := io.ReadAll(f)
	if err != nil {
		return nil, fmt.Errorf("failed to read %q: %w", path, err)
	}
	return data, nil
}

func writeFile(path string, data []byte) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("cannot stat %q: %w", path, err)
	}
	perm := info.Mode().Perm()

	dir := filepath.Dir(path)
	tmp, err := os.CreateTemp(dir, ".qrhex-*.tmp")
	if err != nil {
		return fmt.Errorf("cannot create temp file in %q: %w", dir, err)
	}
	tmpName := tmp.Name()

	committed := false
	defer func() {
		if !committed {
			tmp.Close()
			os.Remove(tmpName)
		}
	}()

	if err := tmp.Chmod(perm); err != nil {
		return fmt.Errorf("cannot set permissions on temp file: %w", err)
	}
	if _, err := tmp.Write(data); err != nil {
		return fmt.Errorf("failed to write temp file: %w", err)
	}
	if err := tmp.Close(); err != nil {
		return fmt.Errorf("failed to close temp file: %w", err)
	}
	if err := os.Rename(tmpName, path); err != nil {
		return fmt.Errorf("failed to replace %q: %w", path, err)
	}

	committed = true
	return nil
}

func printHexDump(data []byte) {
	for rowStart := 0; rowStart < len(data); rowStart += bytesPerRow {
		rowEnd := min(rowStart+bytesPerRow, len(data))
		row := data[rowStart:rowEnd]

		fmt.Printf("%08x  ", rowStart)

		for i, b := range row {
			if i == 8 {
				fmt.Print(" ")
			}
			fmt.Printf("%02x ", b)
		}

		pad := bytesPerRow - len(row)
		if pad > 0 {
			if len(row) <= 8 {
				fmt.Print(" ")
			}
			fmt.Print(strings.Repeat("   ", pad))
		}

		fmt.Print(" |")
		for _, b := range row {
			ch := rune(b)
			if !unicode.IsPrint(ch) {
				ch = '.'
			}
			fmt.Printf("%c", ch)
		}
		fmt.Println("|")
	}

	fmt.Printf("\n%d bytes\n", len(data))
}

func patchByte(data []byte, offset int, val byte) error {
	if offset < 0 || offset >= len(data) {
		return fmt.Errorf("offset %d out of range (file is %d bytes)", offset, len(data))
	}
	data[offset] = val
	return nil
}

func run(argv []string) error {
	a, err := parseArgs(argv)
	if err != nil {
		return err
	}

	data, err := readFile(a.filePath)
	if err != nil {
		return err
	}

	switch a.command {
	case cmdView:
		printHexDump(data)

	case cmdPatch:
		if err := patchByte(data, a.offset, a.byteVal); err != nil {
			return err
		}
		if err := writeFile(a.filePath, data); err != nil {
			return err
		}
		fmt.Printf("patched: offset 0x%08x (%d) -> 0x%02x\n", a.offset, a.offset, a.byteVal)
	}

	return nil
}

func main() {
	if err := run(os.Args); err != nil {
		fmt.Fprintf(os.Stderr, "error: %s\n\n%s", err, usage)
		os.Exit(1)
	}
}
