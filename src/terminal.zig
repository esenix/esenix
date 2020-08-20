const std = @import("std");
const os = std.os;
const c = @import("c.zig").c;

pub const TerminalError = error{ TermiosGetError, TermiosSetError, ReadError, WriteError };

// TODO Types and Docs!
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

    pub fn restoreTermios(self: *Self) TerminalError!void {
        try self.setTermios(&self.stored_termios);
    }

    pub fn enableRawMode(self: *Self) TerminalError!void {
        var termios: c.termios = self.stored_termios;

        termios.c_iflag &= ~@intCast(u16, c.BRKINT | c.ICRNL | c.INPCK | c.ISTRIP | c.IXON);
        termios.c_oflag &= ~@intCast(u16, c.OPOST);
        termios.c_cflag |= @intCast(u16, c.CS8);
        termios.c_lflag &= ~@intCast(u16, c.ECHO | c.ICANON | c.IEXTEN | c.ISIG);

        termios.c_cc[c.VMIN] = 0;
        termios.c_cc[c.VTIME] = 1;

        try self.setTermios(&termios);
    }

    pub fn read(self: *Self) TerminalError!u8 {
        var n: isize = 0;
        var char: u8 = 0;

        while (true) {
            n = c.read(os.STDIN_FILENO, &char, 1);
            if (n == 1)
                return char;
            if (n == -1)
                return error.ReadError;
        }
    }
};
