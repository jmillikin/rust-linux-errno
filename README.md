# Linux error numbers for Rust

This library defines an `Error` struct that represents error numbers
returned from Linux system calls.

On Linux, error numbers are architecture-specific. The `arch` modules
provide access to error numbers for all supported architectures, and the
top-level module re-exports error numbers for the current target platform.

To depend on `linux-errno` from a Bazel workspace:

```python
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rust_posix_errno",
    sha256 = "74e8d0d36c1e6e8c64f0e837f6414c65cf02757a09bdcbf788c04581008f3308",
    urls = ["https://github.com/jmillikin/rust-posix-errno/releases/download/v1.0.0/posix-errno-v1.0.0.tar.xz"],
)

http_archive(
    name = "rust_linux_errno",
    sha256 = "20eddf9456b1d9def6643adf71a3e6d2328f4fbc3063127b4334a710242371a2",
    urls = ["https://github.com/jmillikin/rust-linux-errno/releases/download/v1.0.0/linux-errno-v1.0.0.tar.xz"],
)
```

To depend on `linux-errno` from a Cargo workspace:

```
[dependencies]
linux-errno = { version = "1.0.0" }
```
