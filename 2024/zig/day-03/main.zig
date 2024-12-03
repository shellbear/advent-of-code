const std = @import("std");
const print = std.debug.print;

fn part1(_: std.mem.Allocator, input: []const u8) !u32 {
    var result: u32 = 0;

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        var it = std.mem.tokenizeSequence(u8, line, "mul(");
        while (it.next()) |expression| {
            const position = std.mem.indexOfScalar(u8, expression, ')');
            if (position != null) {
                var parts = std.mem.tokenizeScalar(u8, expression[0..position.?], ',');
                const first = parts.next();
                const second = parts.next();

                if (first != null and second != null) {
                    const first_number = std.fmt.parseInt(u32, first.?, 10) catch {
                        continue;
                    };
                    const second_number = std.fmt.parseInt(u32, second.?, 10) catch {
                        continue;
                    };

                    result += first_number * second_number;
                }
            }
        }
    }

    return result;
}

fn part2(_: std.mem.Allocator, _: []const u8) !u32 {
    return 0;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const file: []const u8 = @embedFile("./day-03.txt");

    const first_result = try part1(allocator, file);
    print("Part 1: {}\n", .{first_result});

    const second_result = try part2(allocator, file);
    print("Part 2: {}\n", .{second_result});
}

test "part_1" {
    const input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const result = try part1(std.testing.allocator, input);
    try std.testing.expectEqual(result, 161);
}

test "part_2" {
    const input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const result = try part2(std.testing.allocator, input);
    try std.testing.expectEqual(result, 31);
}
