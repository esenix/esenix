const std = @import("std");
const os = std.os;
const c = @import("c.zig").c;

pub const Terminal = struct {
    const Self = @This();

    pub fn init() Self {
        var self = Self{
        };

        return self;
    }

};
