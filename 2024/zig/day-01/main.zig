const std = @import("std");
const print = std.debug.print;

fn part1(allocator: std.mem.Allocator, input: []const u8) !u32 {
    var first_list = std.ArrayList(u32).init(allocator);
    defer first_list.deinit();

    var second_list = std.ArrayList(u32).init(allocator);
    defer second_list.deinit();

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        var it = std.mem.tokenizeScalar(u8, line, ' ');

        const first_number = try std.fmt.parseUnsigned(u32, it.next().?, 10);
        const second_number = try std.fmt.parseUnsigned(u32, it.next().?, 10);

        try first_list.append(first_number);
        try second_list.append(second_number);
    }

    std.mem.sort(u32, first_list.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, second_list.items, {}, comptime std.sort.asc(u32));

    var result: u32 = 0;
    for (first_list.items, second_list.items) |first, second| {
        result += if (first > second) first - second else second - first;
    }

    return result;
}

fn part2(allocator: std.mem.Allocator, input: []const u8) !u32 {
    var first_list = std.ArrayList(u32).init(allocator);
    defer first_list.deinit();

    var occurences = std.AutoHashMap(u32, u32).init(allocator);
    defer occurences.deinit();

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        var it = std.mem.tokenizeScalar(u8, line, ' ');

        const first_number = try std.fmt.parseUnsigned(u32, it.next().?, 10);
        const second_number = try std.fmt.parseUnsigned(u32, it.next().?, 10);

        try first_list.append(first_number);

        const entry = try occurences.getOrPut(second_number);
        if (!entry.found_existing) {
            entry.value_ptr.* = 1;
        } else {
            entry.value_ptr.* += 1;
        }
    }

    var result: u32 = 0;
    for (first_list.items) |first| {
        const count = occurences.get(first);
        if (count != null) {
            result += first * count.?;
        }
    }

    return result;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const file: []const u8 = @embedFile("./day-01.txt");

    const first_result = try part1(allocator, file);
    print("Part 1: {}\n", .{first_result});

    const second_result = try part2(allocator, file);
    print("Part 2: {}\n", .{second_result});
}

test "part_1" {
    const input =
        \\ 3   4
        \\ 4   3
        \\ 2   5
        \\ 1   3
        \\ 3   9
        \\ 3   3
    ;

    const result = try part1(std.testing.allocator, input);
    try std.testing.expectEqual(result, 11);
}

test "part_2" {
    const input =
        \\ 3   4
        \\ 4   3
        \\ 2   5
        \\ 1   3
        \\ 3   9
        \\ 3   3
    ;

    const result = try part2(std.testing.allocator, input);
    try std.testing.expectEqual(result, 31);
}
