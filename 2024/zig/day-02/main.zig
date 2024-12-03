const std = @import("std");
const print = std.debug.print;

const Direction = enum {
    up,
    down,
};

fn is_safe(
    array: []const u32,
) bool {
    const direction = if (array[0] > array[1]) Direction.down else Direction.up;

    for (array[0 .. array.len - 1], array[1..]) |first, second| {
        switch (direction) {
            Direction.up => {
                if (first > second or second - first < 1 or second - first > 3) {
                    return false;
                }
            },
            Direction.down => {
                if (first < second or first - second < 1 or first - second > 3) {
                    return false;
                }
            },
        }
    }

    return true;
}

fn is_maybe_safe(
    allocator: std.mem.Allocator,
    array: []u32,
) !bool {
    if (is_safe(array)) {
        return true;
    }

    const elements = std.ArrayList(u32).fromOwnedSlice(allocator, array);

    for (0..array.len) |index| {
        var new_array = try elements.clone();
        _ = new_array.orderedRemove(index);

        if (is_safe(new_array.items)) {
            new_array.deinit();
            return true;
        }

        new_array.deinit();
    }

    return false;
}

fn part1(allocator: std.mem.Allocator, input: []const u8) !u32 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var total: u32 = 0;

    while (lines.next()) |line| {
        var it = std.mem.tokenizeScalar(u8, line, ' ');

        var numbers = std.ArrayList(u32).init(allocator);
        while (it.next()) |number| {
            const parsed = try std.fmt.parseUnsigned(u32, number, 10);
            try numbers.append(parsed);
        }

        if (is_safe(numbers.items)) {
            total += 1;
        }

        numbers.deinit();
    }

    return total;
}

fn part2(allocator: std.mem.Allocator, input: []const u8) !u32 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var total: u32 = 0;

    while (lines.next()) |line| {
        var it = std.mem.tokenizeScalar(u8, line, ' ');

        var numbers = std.ArrayList(u32).init(allocator);
        while (it.next()) |number| {
            const parsed = try std.fmt.parseUnsigned(u32, number, 10);
            try numbers.append(parsed);
        }

        if (try is_maybe_safe(allocator, numbers.items)) {
            total += 1;
        }

        numbers.deinit();
    }

    return total;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const file: []const u8 = @embedFile("./day-02.txt");

    const first_result = try part1(allocator, file);
    print("Part 1: {}\n", .{first_result});

    const second_result = try part2(allocator, file);
    print("Part 2: {}\n", .{second_result});
}

test "part_1" {
    const input =
        \\ 7 6 4 2 1
        \\ 1 2 7 8 9
        \\ 9 7 6 2 1
        \\ 1 3 2 4 5
        \\ 8 6 4 4 1
        \\ 1 3 6 7 9
    ;

    const result = try part1(std.testing.allocator, input);
    try std.testing.expectEqual(result, 2);
}

test "part_2" {
    const input =
        \\ 7 6 4 2 1
        \\ 1 2 7 8 9
        \\ 9 7 6 2 1
        \\ 1 3 2 4 5
        \\ 8 6 4 4 1
        \\ 1 3 6 7 9
    ;

    const result = try part2(std.testing.allocator, input);
    try std.testing.expectEqual(result, 4);
}
