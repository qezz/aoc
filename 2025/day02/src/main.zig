const std = @import("std");

pub fn println(comptime fmt: []const u8, args: anytype) void {
    std.debug.print(fmt ++ "\n", args);
}

fn charToDigit(c: u8) u8 {
    return switch (c) {
        '0'...'9' => c - '0',
        'A'...'Z' => c - 'A' + 10,
        'a'...'z' => c - 'a' + 10,
        else => std.math.maxInt(u8),
    };
}

pub fn parseU64(buf: []const u8, radix: u8) !u64 {
    var x: u64 = 0;

    for (buf) |c| {
        const digit = charToDigit(c);

        if (digit >= radix) {
            return error.InvalidChar;
        }

        // x *= radix
        var ov = @mulWithOverflow(x, radix);
        if (ov[1] != 0) return error.OverFlow;

        // x += digit
        ov = @addWithOverflow(ov[0], digit);
        if (ov[1] != 0) return error.OverFlow;
        x = ov[0];
    }

    return x;
}

pub fn u64ToStr(buf: []u8, x: u64) []const u8 {
    return std.fmt.bufPrint(buf, "{}", .{x}) catch unreachable;
}

const IdRange = struct {
    raw: []const u8,
    from: u64,
    to: u64,

    pub fn fromStr(s: []const u8) IdRange {
        var iter = std.mem.splitScalar(u8, s, '-');

        return IdRange{
            .raw = s,
            .from = parseU64(iter.next().?, 10) catch unreachable,
            .to = parseU64(iter.next().?, 10) catch unreachable,
        };
    }
};

pub fn anyPartsAreDups(s: []const u8) bool {
    for (2..(s.len + 1)) |parts_n| {
        if (s.len % parts_n != 0) {
            continue;
        }

        const win = s.len / parts_n;

        var it = std.mem.window(u8, s, win, win);

        const buf = it.next().?;
        var failed = false;

        while (it.next()) |part| {
            if (!std.mem.eql(u8, buf, part)) {
                failed = true;
                break;
            }
        }

        if (!failed) {
            return true;
        }
    }

    return false;
}

pub fn findDuped(self: IdRange, out: *u64, out2: *u128) void {
    for (self.from..self.to + 1) |x| {
        var b: [20]u8 = undefined;
        const res = u64ToStr(&b, x);

        if (anyPartsAreDups(res)) {
            out2.* += @as(u128, x);
        }

        if (res.len % 2 != 0) {
            continue;
        }

        const win = res.len / 2;

        var it = std.mem.window(u8, res, win, win);

        const half1 = it.next().?;
        const half2 = it.next().?;

        if (std.mem.eql(u8, half1, half2)) {
            out.* += @as(u64, x);
        }
    }
}

pub fn readFile(allocator: std.mem.Allocator, filename: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, std.math.maxInt(usize));
    return contents;
}

pub fn trimStr(input: []const u8) []const u8 {
    const wo_new_lines = std.mem.trim(u8, input, " ");

    return std.mem.trim(u8, wo_new_lines, "\n");
}

pub fn solutions(input: []const u8) void {
    var it = std.mem.splitScalar(u8, input, ',');
    var part1res: u64 = 0;
    var part2res: u128 = 0;

    while (it.next()) |x| {
        const r = IdRange.fromStr(x);
        findDuped(r, &part1res, &part2res);
    }

    println("part1: {}", .{part1res});
    println("part2: {}", .{part2res});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const file_contents = try readFile(allocator, "input.txt");
    defer allocator.free(file_contents);

    const short = trimStr(file_contents);

    solutions(short);
}
