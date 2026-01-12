const std = @import("std");
pub fn fib_largest_lte(n: u64) i64 {
    const x = std.math.sqrt(@as(f64, @floatFromInt(n))) * (@as(f64, @floatFromInt(n)) + 0.5);
    return @as(i64, std.math.floor(std.math.log(f64, std.math.phi, x)));
    // let x = f32::sqrt(5.0) * (n as f32 + 0.5);
    // x.log(std::f32::consts::PHI).floor() as i32
}

test "fib_largest_lte test" {
    try std.testing.expectEqual(0, fib_largest_lte(0));
    try std.testing.expectEqual(2, fib_largest_lte(1));
    try std.testing.expectEqual(4, fib_largest_lte(3));
    try std.testing.expectEqual(6, fib_largest_lte(10));
    try std.testing.expectEqual(7, fib_largest_lte(15));
}
