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
    sha256 = "0c86c849ff673372fe6415d4004a233565b57b2884ea49d3b725dd1296cc2529",
    strip_prefix = "posix-errno-1.0.1",
    urls = ["https://github.com/jmillikin/rust-posix-errno/releases/download/v1.0.1/posix-errno-1.0.1.tar.xz"],
)

http_archive(
    name = "rust_linux_errno",
    # Obtain the package checksum from the release page:
    # https://github.com/jmillikin/rust-linux-errno/releases/tag/v1.0.1
    sha256 = "",
    strip_prefix = "linux-errno-1.0.1",
    urls = ["https://github.com/jmillikin/rust-linux-errno/releases/download/v1.0.1/linux-errno-1.0.1.tar.xz"],
)
```

To depend on `linux-errno` from a Cargo workspace:

```
[dependencies]
linux-errno = { version = "1.0.1" }
```
