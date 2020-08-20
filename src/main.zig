const std = @import("std");
const c = @import("c.zig").c;

const terminal_import = @import("terminal.zig");
const Terminal = terminal_import.Terminal;
const TerminalError = terminal_import.TerminalError;

const program_allocator = std.heap.page_allocator;

pub fn main() !void {
    var terminal = Terminal.init(program_allocator);
    defer terminal.deinit();

    try terminal.storeTermios();
    try terminal.enableRawMode();

    while (true) {
        try terminal.clearScreen();

        try terminal.write(">");
        _ = try terminal.flush();

        var char = try terminal.read();
        if (char == 'q')
            break;
    }

    try terminal.restoreTermios();
}
