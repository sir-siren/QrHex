const std = @import("std");

const BYTES_PER_ROW: usize = 16;
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
const USAGE =
    \\Usage:
    \\  qrhex view  <file>
    \\  qrhex patch <file> <offset_decimal> <byte_hex>
    \\
    \\Examples:
    \\  qrhex view  qr.png
    \\  qrhex patch qr.png 24 ff
    \\
;

const Cmd = enum { view, patch };

const Args = struct {
    cmd: Cmd,
    file: []const u8,
    offset: usize = 0,
    byte_val: u8 = 0,
};

fn parseArgs(argv: [][:0]u8) !Args {
    if (argv.len < 3) return error.NotEnoughArgs;

    const cmd: Cmd = blk: {
        if (std.mem.eql(u8, argv[1], "view")) break :blk .view;
        if (std.mem.eql(u8, argv[1], "patch")) break :blk .patch;
        return error.UnknownCmd;
    };

    const file = argv[2];

    if (cmd == .patch) {
        if (argv.len < 5) return error.NotEnoughArgs;
        const offset = try std.fmt.parseInt(usize, argv[3], 10);
        const byte_val = try std.fmt.parseInt(u8, argv[4], 16);
        return Args{ .cmd = .patch, .file = file, .offset = offset, .byte_val = byte_val };
    }

    return Args{ .cmd = .view, .file = file };
}

fn readFile(alloc: std.mem.Allocator, path: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(path, .{});
    defer file.close();
    return file.readToEndAlloc(alloc, MAX_FILE_SIZE);
}

fn writeFile(alloc: std.mem.Allocator, path: []const u8, data: []const u8) !void {
    const tmp_path = try std.fmt.allocPrint(alloc, "{s}.qrhex.tmp", .{path});
    defer alloc.free(tmp_path);

    {
        const tmp_file = try std.fs.cwd().createFile(tmp_path, .{ .truncate = true });
        errdefer std.fs.cwd().deleteFile(tmp_path) catch {};
        defer tmp_file.close();
        try tmp_file.writeAll(data);
    }

    try std.fs.rename(std.fs.cwd(), tmp_path, std.fs.cwd(), path);
}

fn printHexDump(data: []const u8) !void {
    var buf: [4096]u8 = undefined;
    var w = std.fs.File.stdout().writer(&buf);
    const out: *std.Io.Writer = &w.interface;

    var row_start: usize = 0;
    while (row_start < data.len) : (row_start += BYTES_PER_ROW) {
        const row_end = @min(row_start + BYTES_PER_ROW, data.len);
        const row = data[row_start..row_end];

        try out.print("{x:0>8}  ", .{row_start});

        for (row, 0..) |b, i| {
            if (i == 8) try out.writeAll(" ");
            try out.print("{x:0>2} ", .{b});
        }

        var pad = row.len;
        while (pad < BYTES_PER_ROW) : (pad += 1) {
            if (pad == 8) try out.writeAll(" ");
            try out.writeAll("   ");
        }

        try out.writeAll(" |");
        for (row) |b| {
            const ch: u8 = if (std.ascii.isPrint(b)) b else '.';
            try out.print("{c}", .{ch});
        }
        try out.writeAll("|\n");

        try out.flush();
    }

    try out.print("\n{d} bytes\n", .{data.len});
    try out.flush();
}

fn patchByte(data: []u8, offset: usize, val: u8) !void {
    if (offset >= data.len) return error.OffsetOutOfRange;
    data[offset] = val;
}

fn errorMessage(err: anyerror) []const u8 {
    return switch (err) {
        error.NotEnoughArgs => "not enough arguments",
        error.UnknownCmd => "unknown command",
        error.OffsetOutOfRange => "offset is out of range for this file",
        error.FileNotFound => "file not found",
        error.AccessDenied => "permission denied",
        error.FileBusy => "file is busy",
        error.NoSpaceLeft => "no space left on device",
        error.BadUsage => "",
        else => @errorName(err),
    };
}

fn run(alloc: std.mem.Allocator, argv: [][:0]u8) !void {
    const args = parseArgs(argv) catch |err| {
        const msg = errorMessage(err);
        if (msg.len > 0) {
            std.debug.print("error: {s}\n\n", .{msg});
        }
        std.debug.print("{s}", .{USAGE});
        return error.BadUsage;
    };

    const data = try readFile(alloc, args.file);
    defer alloc.free(data);

    switch (args.cmd) {
        .view => try printHexDump(data),

        .patch => {
            try patchByte(data, args.offset, args.byte_val);
            try writeFile(alloc, args.file, data);

            var buf: [128]u8 = undefined;
            var w = std.fs.File.stdout().writer(&buf);
            const out: *std.Io.Writer = &w.interface;
            try out.print(
                "patched: offset 0x{x:0>8} ({d}) -> 0x{x:0>2}\n",
                .{ args.offset, args.offset, args.byte_val },
            );
            try out.flush();
        },
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const argv = try std.process.argsAlloc(alloc);
    defer std.process.argsFree(alloc, argv);

    run(alloc, argv) catch |err| {
        if (err != error.BadUsage) {
            std.debug.print("error: {s}\n", .{errorMessage(err)});
        }
        std.process.exit(1);
    };
}
