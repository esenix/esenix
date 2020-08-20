const std = @import("std");
const os = std.os;
const c = @import("c.zig").c;

const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub const TerminalError = error{ TermiosGetError, TermiosSetError, ReadError, BufferError, FmtWriteError, BufferError, WinSizeGetError };

// TODO Types and Docs!
pub const Terminal = struct {
    const Self = @This();

    allocator: *Allocator,
    stored_termios: c.termios,
    buffer: ArrayList(u8),

    pub fn init(allocator: *Allocator) Self {
        var self = Self{
            .allocator = allocator,
            .stored_termios = undefined,
            .buffer = ArrayList(u8).init(allocator),
        };

        return self;
    }

    pub fn deinit(self: *Self) void {
        self.buffer.deinit();
        self.* = undefined;
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

    pub fn write(self: *Self, slice: []const u8) TerminalError!void {
        self.buffer.appendSlice(slice) catch |_| return error.WriteError;
    }

    pub fn writeFmt(self: *Self, comptime fmt: []const u8, args: var) TerminalError!void {
        const slice = std.fmt.allocPrint(self.allocator, fmt, args) catch |_| return error.FmtWriteError;
        self.buffer.appendSlice(slice) catch |_| return error.BufferError;
    }

    pub fn flush(self: *Self) TerminalError!isize {
        const slice = self.buffer.toOwnedSlice();

        var result = c.write(os.STDOUT_FILENO, @ptrCast(*const c_void, slice), slice.len);
        if (result == -1)
            return error.WriteError;

        return result;
    }

    // TODO: Implement Fallback
    pub fn getWindowSize(self: *Self) TerminalError![2]u64 {
        var winsize: c.winsize = undefined;

        var result = c.ioctl(os.STDOUT_FILENO, c.TIOCGWINSZ, &winsize);
        if (result == -1 or winsize.ws_col == 0)
            return error.WinSizeGetError;

        return [2]u64{
            winsize.ws_col,
            winsize.ws_row,
        };
    }
};
