const std = @import("std");
const expect = @import("std").testing.expect;
const print = @import("std").debug.print;

pub fn println(comptime fmt: []const u8, args: anytype) void {
    std.debug.print(fmt ++ "\n", args);
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

pub fn findBanksMaxJoltage(line: []const u8) u64 {
    // var max: u64 = 0;
    // var it = std.mem.window(u8, line, 2, 1);

    var debug_max: u64 = 0;
    var debug_max_possible_first: u64 = 0;

    // var pos1: usize = 0;
    var max1: u64 = 0;

    // var pos2: usize = 1;
    var max2: u64 = 0;
    var must_replace_max2 = false;

    for (line, 0..) |c, i| {
        // println("start with: {}", .{max1 * 10 + max2});
        // const x = parseU64(c, 10);
        const x = charToDigit(c);

        if (x > debug_max) {
            debug_max = x;
        }

        if (i != line.len - 1 and x > debug_max_possible_first) {
            debug_max_possible_first = x;
        }

        if (i == 0) {
            max1 = x;
        }
        if (i == 1) {
            max2 = x;
        }

        if (must_replace_max2) {
            max2 = x;
            must_replace_max2 = false;
        }

        const can_replace_first = i != (line.len - 1);

        if (can_replace_first) {
            // println("x vs max1: {} vs {}", .{ x, max1 });
            if (x > max1) {
                max1 = x;
                must_replace_max2 = true;
            } else if (x > max2) {
                max2 = x;
            }
        } else {
            if (x > max2) {
                max2 = x;
            }
        }
        // println("end with: {}", .{max1 * 10 + max2});
    }

    const answer = max1 * 10 + max2;

    if (max1 < debug_max_possible_first) {
        println("max1 < debug_max_possible_first: {} < {}", .{ max1, debug_max_possible_first });
        println("answer: {}", .{answer});
    }

    return answer;
}

pub fn sliceContains(slice: []const u64, item: u64) bool {
    for (slice) |i| {
        if (i == item) {
            return true;
        }
    }

    return false;
}

// pub fn replaceMinWithArgIfGt(line: []const u8, positions: *[12]u64, new_val: u64, new_pos: u64) void {
//     // println("line: {s}, new val: {}", .{ line, new_val });
//     var min_val: u8 = undefined;
//     var remote_min_pos: usize = 0;
//     var local_pos: usize = 0;

//     println("pos before iter: {any}", .{positions});
//     for (positions, 0..) |pos, i| {
//         // println("pos: {}", .{pos});
//         const current_int = charToDigit(line[pos]);
//         // const current_int = line[pos];

//         // if (i == 0) {
//         //     min_val = current_int;
//         //     println("min val: {}", .{min_val});
//         //     // min_pos = pos;
//         //     local_pos = i;
//         // }

//         if (current_int < min_val) {
//             min_val = current_int;
//             // println("min val: {}", .{min_val});
//             // min_pos = pos;
//             local_pos = i;
//             remote_min_pos = pos;
//         }
//     }

//     // println("min val: {}, new val: {}, new val pos: {}", .{ min_val, new_val, new_pos });

//     if (new_val > min_val and !sliceContains(positions, new_pos)) {
//         // buf[min_pos] = x;
//         positions[local_pos] = new_pos;
//     }
//     println("pos after iter : {any}", .{positions});
// }

// pub fn canInsertDataAtPosition(list: []const u64, positions: [12]u64, from_pos: usize, to_pos: usize)

pub fn fetchNumberFrom(line: []const u8, start: usize, length: usize) ?u64 {
    var acc = 0;

    for (start..(start + length)) |i| {
        if (i >= line.len) {
            return null;
        }
        acc = acc * 10 + charToDigit(line[i]);
    }

    return acc;
}

pub fn fetchAtMost(line: []const u8, start: usize, max_len: usize) u64 {
    var acc: u64 = 0;

    for (start..(start + max_len)) |i| {
        if (i >= line.len) {
            return acc;
        }

        acc = acc * 10 + charToDigit(line[i]);
    }

    return acc;
}

pub fn findIndexAtWhichItsGtAndUpdate(line: []const u8, positions: *[12]u64, line_pos: usize, new_val: u64) void {
    expect(line_pos < line.len) catch unreachable;
    // println("looking at {} (val {}), new val: {}", .{ line_pos, charToDigit(line[line_pos]), new_val });
    // println()
    println("{s}", .{line});
    for (0..line_pos) |_| {
        print("-", .{});
    }
    println("^", .{});
    println("looking at line[{}] : {}", .{ line_pos, new_val });

    const global_at_most = line.len - line_pos;
    println("global at most: {}", .{global_at_most});

    for (positions, 0..) |pos, i| {
        if (line_pos <= pos) {
            return;
        }
        const required_here = 12 - i;
        println("at positions[{}] :: {}, required chars: {}", .{ i, pos, required_here });

        if (required_here > global_at_most) {
            continue;
        }

        const r1 = charToDigit(line[pos]);
        println("looking at pos: {} (rel: {}), curr: {} -> {}?", .{ pos, i, r1, new_val });

        if (new_val > r1) {
            // const at_most = 12 - i;

            // const new_at_most = @min(global_at_most, at_most);
            // println("new at most: {}", .{new_at_most});

            // var n = fetchAtMost(line, pos, new_at_most);

            for (i..(i + required_here), 0..) |j, e| {
                println("j: {} | line pos: {}", .{ j, line_pos });
                // positions[j] = n % 10;
                // n = n / 10;

                positions[j] = line_pos + e;
            }

            return;
        }
    }
}

// pub fn updatePositions(line: []const u8, positions: *[12]u64, new_val: u64, new_pos: u64) void {}

pub fn current(line: []const u8, positions: [12]u64) u64 {
    var acc: u64 = 0;

    for (positions) |pos| {
        acc = acc * 10 + charToDigit(line[pos]);
    }

    return acc;
}

pub fn findBanksMaxJoltage12(line: []const u8) u64 {
    println("line: {s}", .{line});
    var positions: [12]u64 = [_]u64{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 };

    println("pos before: {any}", .{positions});
    for (line, 0..) |c, c_pos| {
        // if (c_pos < 12) {
        //     continue;
        // }

        const v = charToDigit(c);
        // updatePositions(line, &positions, v, c_pos);

        println("\n  before iter: {any}", .{positions});
        findIndexAtWhichItsGtAndUpdate(line, &positions, c_pos, v);
        println("   after iter: {any}", .{positions});

        println("\ncurrent: {}\n\n", .{current(line, positions)});
    }
    println("pos after: {any}", .{positions});

    // std.mem.sort(u64, &positions, {}, comptime std.sort.asc(u64));
    // println("pos sorted: {any}", .{positions});
    var acc: u64 = 0;

    for (positions) |pos| {
        acc = acc * 10 + charToDigit(line[pos]);
    }

    println("data: {any}", .{acc});

    return acc;
}

pub fn solutions(input: []const u8) void {
    var lines_it = std.mem.splitScalar(u8, input, '\n');

    var sum: u64 = 0;
    var sum2: u64 = 0;

    while (lines_it.next()) |line| {
        println("{s}", .{line});
        const res = findBanksMaxJoltage(line);
        println(" > {}", .{res});
        sum += res;

        const res2 = findBanksMaxJoltage12(line);
        println(" >> {}", .{res2});
        sum2 += res2;
    }

    println("part1: {}", .{sum});
    println("part2: {}", .{sum2});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const file_contents = try readFile(allocator, "input.txt");
    defer allocator.free(file_contents);

    const short = trimStr(file_contents);

    solutions(short);
}

test "part1 test1" {
    const input = "81819121";
    const actual = findBanksMaxJoltage(input);
    // println("actual: {}", .{actual});
    try expect(actual == 92);
}
test "part1 test2" {
    const input = "92";
    const actual = findBanksMaxJoltage(input);
    // println("actual: {}", .{actual});
    try expect(actual == 92);
}
test "part1 test3" {
    const input = "29";
    const actual = findBanksMaxJoltage(input);
    // println("actual: {}", .{actual});
    try expect(actual == 29);
}
test "part1 test4" {
    const input = "8111119";
    const actual = findBanksMaxJoltage(input);
    // println("actual: {}", .{actual});
    try expect(actual == 89);
}
test "part1 test5" {
    const input = "111111119";
    const actual = findBanksMaxJoltage(input);
    // println("actual: {}", .{actual});
    try expect(actual == 19);
}
test "part1 test6" {
    const input = "119111119";
    const actual = findBanksMaxJoltage(input);
    // println("actual: {}", .{actual});
    try expect(actual == 99);
}

test "part1 test7" {
    const input = "144456789";
    const actual = findBanksMaxJoltage(input);

    // println("actual: {}", .{actual});
    try expect(actual == 89);
}

test "part2 test1" {
    // const input = "112233446677";
    // _ = findBanksMaxJoltage12(input);

    // println("actual: {}", .{actual});
    // try expect(actual == 89);
}

test "part2 test2" {
    // const input = "6655443322111119999";
    // _ = findBanksMaxJoltage12(input);

    // println("actual: {}", .{actual});
    // try expect(actual == 89);
}

test "part2 test3" {
    const input = "234234234234278";
    _ = findBanksMaxJoltage12(input);
}

// test "part2 test4" {xxx
//     const input
// }
