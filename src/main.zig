const std = @import("std");
const c = @import("c.zig").c;

const terminal_import = @import("terminal.zig");
const Terminal = terminal_import.Terminal;
const TerminalError = terminal_import.TerminalError;

pub fn main() !void {
    var terminal = Terminal.init();
    try terminal.storeTermios();
}
