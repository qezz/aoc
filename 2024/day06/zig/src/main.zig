const std = @import("std");

const Pos = struct {
    row: usize,
    col: usize,

    pub fn format(
        self: Pos,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        try writer.print("({d}, {d})", .{ self.row, self.col });
    }
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

    pub fn turn_right(self: Guard) Guard {
        return switch (self) {
            .n => Guard.e,
            .e => Guard.s,
            .s => Guard.w,
            .w => Guard.n,
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

    pub fn vacant(self: CellT) bool {
        return switch (self) {
            CellT.busy => false,
            CellT.guard => unreachable,
            else => true,
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
    pos: Pos,
    t: CellT,

    pub fn init(row: usize, col: usize, t: CellT) Cell {
        return Cell{
            .pos = Pos{ .row = row, .col = col },
            .t = t,
        };
    }

    pub fn vacant(self: Cell) bool {
        return self.t.vacant();
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

const Vm = struct {
    map: Map,
    guard_dir: Guard,
    guard_pos: Pos,

    pub fn init(map: Map) Vm {
        var guard_dir: Guard = .n;
        var guard_pos: ?Pos = null;

        for (map.inner.items, 0..) |row, rowi| {
            for (row.items, 0..) |cell, coli| {
                switch (cell.t) {
                    CellT.guard => {
                        std.log.debug("guard: {d} {d} {s}", .{ rowi, coli, cell.t });
                        map.inner.items[rowi].items[coli].t = .visited;
                        std.log.debug("guard: {d} {d} {s}", .{ rowi, coli, map.inner.items[rowi].items[coli].t });

                        guard_dir = cell.t.guard;
                        guard_pos = cell.pos;
                    },
                    else => {},
                }
            }
        }

        return Vm{ .map = map, .guard_dir = guard_dir, .guard_pos = guard_pos.? };
    }

    pub fn get(self: Vm, row: usize, col: usize) Cell {
        return self.map.inner.items[row].items[col];
    }

    pub fn get_mut(self: *Vm, row: usize, col: usize) *Cell {
        return self.map.inner.items[row].items[col];
    }

    pub fn get_pos_mut(self: *Vm, pos: Pos) *Cell {
        return self.map.inner.items[pos.row].items[pos.col];
    }

    pub fn maybe_get_pos(self: Vm, pos: Pos) ?Cell {
        if (pos.row < self.map.inner.items.len and pos.col < self.map.inner.items[pos.row].items.len) {
            return self.map.inner.items[pos.row].items[pos.col];
        }

        return null;
    }

    pub fn peek_next_cell_pos(self: Vm) Pos {
        const d = self.guard_dir;
        const p = self.guard_pos;

        return switch (d) {
            Guard.n => .{ .row = p.row - 1, .col = p.col },
            Guard.e => .{ .row = p.row, .col = p.col + 1 },
            Guard.s => .{ .row = p.row + 1, .col = p.col },
            Guard.w => .{ .row = p.row, .col = p.col - 1 },
        };
    }

    pub fn peek_next_cell_value(self: Vm) CellT {
        const next = self.peek_next_cell_pos();
        const cell = self.get(next.row, next.col);

        std.log.debug("cell: {s}", .{cell.t});

        return cell.t;
    }

    pub fn count_visited(self: Vm) usize {
        var total: usize = 0;

        for (self.map.inner.items) |row| {
            for (row.items) |cell| {
                switch (cell.t) {
                    .visited => total += 1,
                    else => {},
                }
            }
        }

        return total;
    }

    pub fn tick(self: *Vm) bool {
        const next = self.peek_next_cell_pos();
        const maybe_cell: ?Cell = self.maybe_get_pos(next);

        if (maybe_cell) |cell| {
            if (cell.vacant()) {
                std.log.debug("cell: {s} vacant", .{cell.t});
                self.guard_pos = next;

                var mut_cell = &self.map.inner.items[next.row].items[next.col];
                mut_cell.t = .visited;

                return true;
            } else {
                self.guard_dir = self.guard_dir.turn_right();

                return true;
            }
        }

        return false;
    }

    pub fn format(
        self: Vm,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;

        try writer.print("guard: {s} ", .{self.guard_pos});
        try writer.print("{s}\n", .{self.guard_dir});
        try writer.print("{s}", .{self.map});
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

    const m = try Map.from_str(file_contents, allocator);
    try std.io.getStdOut().writer().print("Map:\n{any}\n", .{m});

    var vm = Vm.init(m);
    try std.io.getStdOut().writer().print("Vm:\n{any}\n", .{vm});

    const next_p = vm.peek_next_cell_pos();
    try std.io.getStdOut().writer().print("Next: {any}\n", .{next_p});

    const next_v = vm.peek_next_cell_value();
    try std.io.getStdOut().writer().print("Next: {any}\n", .{next_v});

    var can_continue = true;
    while (can_continue) {
        can_continue = vm.tick();
        try std.io.getStdOut().writer().print("Vm:\n{any}\n", .{vm});
    }

    const visited = vm.count_visited();
    try std.io.getStdOut().writer().print("Visited:\n{d}\n", .{visited});
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
