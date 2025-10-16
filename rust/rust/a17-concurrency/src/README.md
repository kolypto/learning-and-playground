# Rust/Go FFI Performance

What's wrong with FFI?
CGo uses a separate thread for FFI, which involves copying parameters back and forth.
The overhead is quite significant. If you're making a lot of FFI calls, e.g. interfacing
with a C library to process multimedia .. Go is the wrong language.
Its CGo is 2x slower than Rust. Benchmarks show CGo can add 50-100ns per call vs Rust's ~10ns.
Because in Rust, an FFI is just a direct function call.

Bottom line: yes, if you're doing heavy FFI (game engines, media processing, database drivers),
Rust's zero-cost FFI is a real advantage over CGo. For occasional FFI calls, doesn't matter much.

Same applies to syscalls: because they're FFI calls into the kernel.
In Go, every syscall has CGo-like overhead: the runtime has to coordinate goroutine scheduling
around blocking syscalls. This is why Rust can be faster for I/O-heavy workloads
that aren't using async. Lots of read(), write(), open() syscalls? Rust's direct approach wins.
Go mitigates this with its async I/O under the hood (syscalls go through the netpoller
when possible), but the runtime overhead is still there. For most applications this doesn't
matter—network latency dwarfs syscall overhead. But for high-performance systems —
Rust is measurably faster.
