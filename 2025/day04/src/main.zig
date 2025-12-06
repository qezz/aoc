const std = @import("std");

pub fn println(comptime fmt: []const u8, args: anytype) void {
    std.debug.print(fmt ++ "\n", args);
}

pub fn trimStrSlice(input: []u8) []u8 {
    _ = std.mem.trim(u8, input, " ");

    return input;
}

pub fn trimStr(input: []const u8) []const u8 {
    const wo_new_lines = std.mem.trim(u8, input, " ");

    return std.mem.trim(u8, wo_new_lines, "\n");
}

pub fn readFile(allocator: std.mem.Allocator, filename: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    const contents = try file.readToEndAlloc(allocator, std.math.maxInt(usize));
    return contents;
}

pub fn sliceGet(comptime T: type, s: []T, idx: usize) ?T {
    if (idx >= s.len) {
        return null;
    }

    return s[idx];
}

pub fn inputGet(comptime T: type, s: []const T, width: usize, i: usize, j: usize) ?T {
    // +1 for \n
    const true_offset = (width + 1) * i + j;

    if (true_offset >= s.len) {
        return null;
    }

    return s[true_offset];
}

pub fn inputRemoveAt(comptime T: type, s: []T, width: usize, i: usize, j: usize) bool {
    // +1 for \n
    const true_offset = (width + 1) * i + j;

    if (true_offset >= s.len) {
        return false;
    }

    s[true_offset] = '.';
    return true;
}
pub fn part1(input: []const u8) usize {
    var acc: u64 = 0;
    var lines = std.mem.splitScalar(u8, input, '\n');

    var i: usize = 0;

    while (lines.next()) |raw_line| {
        const line = trimStr(raw_line);

        for (line, 0..) |_, j| {
            var neigh: usize = 0;
            const raw = inputGet(u8, input, line.len, i, j);

            if (raw orelse '.' != '@') {
                continue;
            }

            const i_from: usize = @max(@as(isize, @intCast(i)) - 1, 0);
            const j_from: usize = @max(@as(isize, @intCast(j)) - 1, 0);

            for (i_from..i + 2) |ni| {
                for (j_from..j + 2) |nj| {
                    if (ni == i and nj == j) {
                        continue;
                    }

                    const thing = inputGet(u8, input, line.len, ni, nj) orelse '.';

                    if (thing == '@') {
                        neigh += 1;
                    }
                }
            }

            if (neigh < 4) {
                acc += 1;
            }
        }

        i += 1;
    }

    println("acc: {}", .{acc});

    return acc;
}

pub fn removeAccessibleOnce(input: []u8) usize {
    var total: usize = 0;
    var lines = std.mem.splitScalar(u8, input, '\n');

    var i: usize = 0;

    while (lines.next()) |raw_line| {
        const line = trimStr(raw_line);
        for (line, 0..) |_, j| {
            var neigh: usize = 0;
            const raw = inputGet(u8, input, line.len, i, j);

            if (raw orelse '.' != '@') {
                continue;
            }

            const i_from: usize = @max(@as(isize, @intCast(i)) - 1, 0);
            const j_from: usize = @max(@as(isize, @intCast(j)) - 1, 0);

            for (i_from..i + 2) |ni| {
                for (j_from..j + 2) |nj| {
                    if (ni == i and nj == j) {
                        continue;
                    }

                    const thing = inputGet(u8, input, line.len, ni, nj) orelse '.';

                    if (thing == '@') {
                        neigh += 1;
                    }
                }
            }

            if (neigh < 4) {
                const res = inputRemoveAt(u8, input, line.len, i, j);
                if (res) {
                    total += 1;
                }
            }
        }

        i += 1;
    }

    return total;
}

pub fn part2(input: []u8) usize {
    var acc: usize = 0;
    while (true) {
        const res = removeAccessibleOnce(input);
        if (res == 0) {
            break;
        }

        acc += res;
    }

    println("acc: {}", .{acc});

    return acc;
}

pub fn solutions(input: []u8) void {
    _ = part1(input);
    _ = part2(input);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const file_contents: []u8 = try readFile(allocator, "input.txt");
    defer allocator.free(file_contents);

    const short = trimStrSlice(file_contents);

    solutions(short);
}

// test "part1 test1" {
//     const input = "..@\n...\n...";
//     part1(input);
// }
// test "part1 test2" {
//     const input = "..@\n.@.\n@..";
//     part1(input);
// }
test "part1 test3" {
    const input_literal = "@@@\n@@@\n@@@";
    // var input_l: []u8 = "@@@\n@@@\n@@@";
    // var input = input_literal[0..];
    // var input = std.mem.bytesAsSlice([]u8, input_literal);
    // part1(input_l);
    var input: [100]u8 = undefined;
    _ = try std.fmt.bufPrint(&input, "{s}", .{input_literal});
}
