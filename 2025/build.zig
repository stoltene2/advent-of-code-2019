const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{
        .name = "day01",
        .root_module = b.createModule(.{
            .root_source_file = b.path("day01/day01.zig"),
            .target = b.graph.host,
        }),
    });

    b.installArtifact(exe);
}

// const std = @import("std");

// pub fn build(b: *std.Build) error{ AccessDenied, InvalidUtf8, PermissionDenied, SystemResources, Unexpected }!void {
//     const target = b.standardTargetOptions(.{});
//     const optimize = b.standardOptimizeOption(.{});

//     // --- Scan the project root for dayXX folders ---
//     const allocator = b.allocator;
//     var cwd = std.fs.cwd();

//     var contents = cwd.iterate();
//     while (try contents.next()) |entry| {
//         if (entry.kind != .directory) continue;

//         const name = entry.name;
//         // Match "day01".."day25"
//         if (!(name.len == 5 and std.mem.startsWith(u8, name, "day"))) continue;

//         // Ensure digits
//         const digits = name[3..5];
//         if (!std.ascii.isDigit(digits[0]) or !std.ascii.isDigit(digits[1])) continue;

//         // Path to source file: dayXX/dayXX.zig
//         const path = std.fmt.allocPrint(allocator, "{s}/{s}.zig", .{ name, name }) catch continue;

//         // Add executable
//         const exe = b.addExecutable(.{
//             .name = name,
//             .root_source_file = .{ .path = path },
//             .target = target,
//             .optimize = optimize,
//         });

//         exe.install();

//         // Add a `zig build dayXX` step
//         const run_step = b.step(name, "Build day executable");
//         run_step.dependOn(&exe.install_step);
//     }
// }
