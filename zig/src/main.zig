const std = @import("std");
const num = @import("num");
const Io = std.Io;

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_file_writer: Io.File.Writer = .init(.stdout(), io, &stdout_buffer);
    const stdout_writer = &stdout_file_writer.interface;

    try stdout_writer.print("P0 {} \n", .{p0(u64, 554_000)});
    try stdout_writer.print("P1 {} \n", .{p1(u64, 1_000)});

    try stdout_writer.flush(); // Don't forget to flush!
}

fn p0(comptime T: type, n: T) T {
    var sum: T = 0;
    const start: T = 1;
    for (start..n) |x| {
        sum += if (x % 2 == 1) x * x else 0;
    }
    return sum;
}

fn p1(comptime T: type, n: T) T {
    var sum: T = 0;
    const start: T = 1;
    for (start..n) |x| {
        sum += if (x % 3 == 0 or x % 5 == 0) x else 0;
    }
    return sum;
}

test "p0 test" {
    try std.testing.expectEqual(28_338_577_333_241_000, p0(u64, 554_000));
}

test "p1 test" {
    try std.testing.expectEqual(233_168, p1(u64, 1000));
}
