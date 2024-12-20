const std = @import("std");
const print = std.debug.print;

const WORD_LOOKUP = "XMAS";

const Position = struct {
    x: usize,
    y: usize,
};

const ALL_DIRECTIONS = [_][2]i8{
    .{ 0, -1 }, // Up
    .{ 0, 1 }, // Down
    .{ -1, 0 }, // Left
    .{ 1, 0 }, // Right
    .{ -1, -1 }, // Up Left
    .{ -1, 1 }, // Up right
    .{ 1, -1 }, // Down left
    .{ 1, 1 }, // Down right
};

fn check_for_word(grid: []const []const u8, position: Position, direction: [2]i8) !bool {
    const rows: i32 = @intCast(grid.len);
    const cols: i32 = @intCast(grid[0].len);

    const row_direction: i64 = @intCast(direction[0]);
    const col_direction: i64 = @intCast(direction[1]);
    const row_max: i64 = @intCast(position.x + WORD_LOOKUP.len);
    const col_max: i64 = @intCast(position.y + WORD_LOOKUP.len);

    if (row_max * row_direction < 0 or row_max * row_direction >= rows or col_max * col_direction < 0 or col_max * col_direction >= cols) {
        return false;
    }

    for (0..WORD_LOOKUP.len) |i| {
        const pos: i64 = @intCast(i);
        const r: usize = @intCast(pos * row_direction);
        const c: usize = @intCast(pos * col_direction);

        if (grid[position.x + r][position.y + c] != WORD_LOOKUP[i]) {
            return false;
        }
    }

    return true;
}

fn part1(allocator: std.mem.Allocator, input: []const u8) !u32 {
    var lines = std.ArrayList([]const u8).init(allocator);
    defer lines.deinit();

    var it = std.mem.tokenize(u8, input, "\n");
    while (it.next()) |line| {
        try lines.append(line);
    }

    var count: u32 = 0;
    for (0..lines.items.len) |i| {
        for (0..lines.items[i].len) |j| {
            for (ALL_DIRECTIONS) |direction| {
                if (try check_for_word(lines.items, .{ .x = i, .y = j }, direction)) {
                    count += 1;
                }
            }
        }
    }

    return count;
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
    const input =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;

    const result = try part1(std.testing.allocator, input);
    try std.testing.expectEqual(18, result);
}

// test "part_2" {
//     const input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
//     const result = try part2(std.testing.allocator, input);
//     try std.testing.expectEqual(result, 31);
// }
