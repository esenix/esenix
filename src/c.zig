pub const c = @cImport({
    @cDefine("_DEFAULT_SOURCE", {});
    @cDefine("_BSD_SOURCE", {});
    @cDefine("_GNU_SOURCE", {});
    @cInclude("termios.h");
    @cInclude("unistd.h");
    @cInclude("sys/ioctl.h");
});
