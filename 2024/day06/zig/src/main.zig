const std = @import("std");

const Point = struct {
    row: usize,
    col: usize,
};

const Guard = union(enum) {
    n,
    e,
    s,
    w,

    pub fn from_char(c: u8) Guard {
        return switch (c) {
            '^' => Guard.n,
            'v' => Guard.s,
            '>' => Guard.e,
            '<' => Guard.w,
            else => unreachable,
        };
    }

    pub fn format(
        self: Guard,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        switch (self) {
            Guard.n => try writer.print("^", .{}),
            Guard.e => try writer.print(">", .{}),
            Guard.s => try writer.print("v", .{}),
            Guard.w => try writer.print("<", .{}),
        }
    }
};

const CellT = union(enum) {
    empty,
    busy,
    guard: Guard,
    visited,

    pub fn from_char(c: u8) CellT {
        return switch (c) {
            '#' => CellT.busy,
            '.' => CellT.empty,
            'X' => CellT.visited,
            else => CellT{ .guard = Guard.from_char(c) },
        };
    }

    pub fn format(
        self: CellT,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        switch (self) {
            CellT.empty => try writer.print(".", .{}),
            CellT.busy => try writer.print("#", .{}),
            CellT.guard => try writer.print("^", .{}),
            CellT.visited => try writer.print("X", .{}),
        }
    }
};

const Cell = struct {
    pos: Point,
    t: CellT,

    pub fn init(row: usize, col: usize, t: CellT) Cell {
        return Cell{
            .pos = Point{ .row = row, .col = col },
            .t = t,
        };
    }
};

const Map = struct {
    inner: std.ArrayList(std.ArrayList(Cell)),

    pub fn from_str(data: []u8, allocator: std.mem.Allocator) !Map {
        var linesIter = std.mem.split(u8, data, "\n");
        var rows = std.ArrayList(std.ArrayList(Cell)).init(allocator);

        var row: usize = 0;

        while (linesIter.next()) |line| {
            var col: usize = 0;
            var cols = std.ArrayList(Cell).init(allocator);

            for (line) |char| {
                const c = CellT.from_char(char);
                try cols.append(Cell.init(row, col, c));

                col += 1;
            }

            if (cols.items.len != 0) {
                try rows.append(cols);
                row += 1;
            }
        }

        return Map{ .inner = rows };
    }

    pub fn format(
        self: Map,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        for (self.inner.items) |row| {
            for (row.items) |cell| {
                try writer.print("{c}", .{cell.t});
            }
            try writer.print("\n", .{});
        }
    }
};

pub fn read_file(allocator: std.mem.Allocator, filename: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, std.math.maxInt(usize));
    return contents;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const file_contents = try read_file(allocator, "../test.txt");
    defer allocator.free(file_contents);

    const m = Map.from_str(file_contents, allocator);
    try std.io.getStdOut().writer().print("Map:\n{any}\n", .{m});
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "read file" {
    const file = try std.fs.cwd().openFile(
        "./zigtest.txt",
        .{},
    );

    try file.seekTo(0);
    const content = try file.readToEndAlloc(std.testing.allocator, std.math.maxInt(usize));
    defer std.testing.allocator.free(content);

    try std.testing.expect(std.mem.eql(u8, content, "hello\n"));
}
