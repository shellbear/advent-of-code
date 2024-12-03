const std = @import("std");
const print = std.debug.print;

fn part1(input: []const u8) !u32 {
    // You'll need an allocator for dynamic lists
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

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

pub fn main() !void {
    const file: []const u8 = @embedFile("./day-01.txt");
    const result = try part1(file);
    print("Result: {}\n", .{result});
}

test "example" {
    const input =
        \\ 3   4
        \\ 4   3
        \\ 2   5
        \\ 1   3
        \\ 3   9
        \\ 3   3
    ;

    try std.testing.expect(try part1(input) == 11);
}
