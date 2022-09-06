workspace(name = "rust_linux_errno")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "0cc7e6b39e492710b819e00d48f2210ae626b717a3ab96e048c43ab57e61d204",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.10.0/rules_rust-v0.10.0.tar.gz"],
)

http_archive(
    name = "rust_posix_errno",
    sha256 = "74e8d0d36c1e6e8c64f0e837f6414c65cf02757a09bdcbf788c04581008f3308",
    urls = ["https://github.com/jmillikin/rust-posix-errno/releases/download/v1.0.0/posix-errno-v1.0.0.tar.xz"],
)

load(
    "@rules_rust//rust:repositories.bzl",
    "rules_rust_dependencies",
    "rust_register_toolchains",
)

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2018",
    version = "1.63.0",
)
