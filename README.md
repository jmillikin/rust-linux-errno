# Linux error numbers for Rust

This library defines an `Error` struct that represents error numbers
returned from Linux system calls.

On Linux, error numbers are architecture-specific. The `arch` modules
provide access to error numbers for all supported architectures, and the
top-level module re-exports error numbers for the current target platform.
