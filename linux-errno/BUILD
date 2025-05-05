load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_library")

rust_library(
    name = "linux-errno",
    srcs = glob([
        "linux-errno.rs",
        "linux-errno_*.rs",
    ]),
    crate_features = ["posix-traits"],
    edition = "2018",
    target_compatible_with = [
        "@platforms//os:linux",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "@rust_posix_errno//posix-errno",
    ],
)

rust_doc(
    name = "linux-errno_doc",
    crate = ":linux-errno",
)
