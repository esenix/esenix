const std = @import("std");
const os = std.os;
const c = @import("c.zig").c;

pub const TerminalError = error{ TermiosGetError, TermiosSetError };

pub const Terminal = struct {
    const Self = @This();

    stored_termios: c.termios,

    pub fn init() Self {
        var self = Self{
            .stored_termios = undefined,
        };

        return self;
    }

    pub fn setTermios(self: *Self, termios: *c.termios) TerminalError!void {
        var result = c.tcsetattr(os.STDIN_FILENO, c.TCSAFLUSH, termios);
        if (result == -1)
            return error.TermiosSetError;
    }

    pub fn storeTermios(self: *Self) TerminalError!void {
        var result = c.tcgetattr(os.STDIN_FILENO, &self.stored_termios);
        if (result == -1)
            return error.TermiosGetError;
    }

    pub fn restoreTermios() TerminalError!void {
        try self.setTermios(&self.stored_termios);
    }
};
